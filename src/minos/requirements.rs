use crate::{authorization::Actor, errors::Error, minos::lang::Indentifier};

use super::lang::{
    ListValueAttribute, ListValueOperator, SingleValueAttribute, SingleValueOperator, Token,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Requirement {
    SingleValue {
        attribute: SingleValueAttribute,
        operator: SingleValueOperator,
        value: String,
    },
    ListValue {
        attribute: ListValueAttribute,
        operator: ListValueOperator,
        value: Vec<String>,
    },
}

impl Requirement {
    fn single_value_from_tokens(tokens: &Vec<Token>) -> Self {
        let Indentifier(value) = tokens[2].inner_identifier().unwrap();
        Self::SingleValue {
            attribute: tokens[0].inner_single_value_attribute().unwrap(),
            operator: tokens[1].inner_single_value_operator().unwrap(),
            value: value.to_string(),
        }
    }

    fn list_value_from_tokens(tokens: &Vec<Token>) -> Self {
        Self::ListValue {
            attribute: tokens[0].inner_list_value_attribute().unwrap(),
            operator: tokens[1].inner_list_value_operator().unwrap(),
            value: tokens[2]
                .inner_array()
                .unwrap()
                .0
                .iter()
                .map(|v| v.to_string())
                .collect(),
        }
    }

    fn apply_sinlge_value(
        actor: &impl Actor,
        attribute: &SingleValueAttribute,
        operator: &SingleValueOperator,
        value: &String,
    ) -> bool {
        match attribute {
            SingleValueAttribute::Type => Self::compare_type(actor, operator, value),
            SingleValueAttribute::Id => Self::compare_id(actor, operator, value),
        }
    }

    fn compare_type(actor: &impl Actor, operator: &SingleValueOperator, value: &String) -> bool {
        match operator {
            SingleValueOperator::Equal => &actor.actor_type() == value,
            SingleValueOperator::Distinct => &actor.actor_type() != value,
        }
    }

    fn compare_id(actor: &impl Actor, operator: &SingleValueOperator, value: &String) -> bool {
        match operator {
            SingleValueOperator::Equal => &actor.actor_id() == value,
            SingleValueOperator::Distinct => &actor.actor_id() != value,
        }
    }

    fn apply_list_value(
        actor: &impl Actor,
        attribute: &ListValueAttribute,
        operator: &ListValueOperator,
        value: &Vec<String>,
    ) -> bool {
        match attribute {
            ListValueAttribute::Groups => Self::compare_groups(actor, operator, value),
            ListValueAttribute::Roles => Self::compare_roles(actor, operator, value),
        }
    }

    fn compare_groups(
        actor: &impl Actor,
        operator: &ListValueOperator,
        value: &Vec<String>,
    ) -> bool {
        match operator {
            ListValueOperator::Equal => &actor.actor_groups() == value,
            ListValueOperator::Contains => Self::find_in_list(&actor.actor_groups(), value),
        }
    }

    fn compare_roles(
        actor: &impl Actor,
        operator: &ListValueOperator,
        value: &Vec<String>,
    ) -> bool {
        match operator {
            ListValueOperator::Equal => &actor.actor_roles() == value,
            ListValueOperator::Contains => Self::find_in_list(&actor.actor_roles(), value),
        }
    }

    fn find_in_list(actor_list: &Vec<String>, to_find: &Vec<String>) -> bool {
        for to_find_item in to_find {
            if !actor_list.contains(to_find_item) {
                return false;
            }
        }

        true
    }

    pub fn apply(&self, actor: &impl Actor) -> bool {
        match self {
            Requirement::SingleValue {
                attribute,
                operator,
                value,
            } => Self::apply_sinlge_value(actor, attribute, operator, value),
            Requirement::ListValue {
                attribute,
                operator,
                value,
            } => Self::apply_list_value(actor, attribute, operator, value),
        }
    }
}

impl TryFrom<&Token<'_>> for Requirement {
    type Error = Error;

    fn try_from(token: &Token<'_>) -> Result<Self, Self::Error> {
        let inner_tokens = token.inner_requirement().ok_or(Error::InvalidToken {
            expected: Token::Requirement(vec![]).to_string(),
            found: token.to_string(),
        })?;

        match &inner_tokens[0] {
            Token::SingleValueRequirement(tokens) => Ok(Self::single_value_from_tokens(tokens)),
            Token::ListValueRequirement(tokens) => Ok(Self::list_value_from_tokens(tokens)),
            _ => unreachable!(),
        }
    }
}
