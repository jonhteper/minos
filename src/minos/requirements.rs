use crate::{
    authorization::{Actor, Resource},
    errors::Error,
    minos::parser::tokens::Indentifier,
};

use super::parser::tokens::{
    ListValueAttribute, ListValueOperator, ResourceAttribute, SingleValueAttribute,
    SingleValueOperator, Token,
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
    AttributesComparation {
        left_attribute: SingleValueAttribute,
        operator: SingleValueOperator,
        right_attribute: ResourceAttribute,
    },
}

impl Requirement {
    fn single_value_from_tokens(tokens: &[Token]) -> Self {
        let Indentifier(value) = tokens[2].inner_identifier().unwrap();
        Self::SingleValue {
            attribute: tokens[0].inner_single_value_attribute().unwrap(),
            operator: tokens[1].inner_single_value_operator().unwrap(),
            value: value.to_string(),
        }
    }

    fn list_value_from_tokens(tokens: &[Token]) -> Self {
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

    fn attributes_comparation_from_tokens(tokens: &[Token]) -> Self {
        Self::AttributesComparation {
            left_attribute: tokens[0].inner_single_value_attribute().unwrap(),
            operator: tokens[1].inner_single_value_operator().unwrap(),
            right_attribute: tokens[2].inner_resource_attribute().unwrap(),
        }
    }

    fn apply_sinlge_value(
        actor: &Actor,
        attribute: &SingleValueAttribute,
        operator: &SingleValueOperator,
        value: &String,
    ) -> bool {
        match attribute {
            SingleValueAttribute::Type => Self::compare_type(actor, operator, value),
            SingleValueAttribute::Id => Self::compare_id(actor, operator, value),
        }
    }

    fn compare_type(actor: &Actor, operator: &SingleValueOperator, value: &String) -> bool {
        match operator {
            SingleValueOperator::Equal => actor.actor_type() == value,
            SingleValueOperator::Distinct => actor.actor_type() != value,
        }
    }

    fn compare_id(actor: &Actor, operator: &SingleValueOperator, value: &String) -> bool {
        match operator {
            SingleValueOperator::Equal => actor.actor_id() == value,
            SingleValueOperator::Distinct => actor.actor_id() != value,
        }
    }

    fn apply_list_value(
        actor: &Actor,
        attribute: &ListValueAttribute,
        operator: &ListValueOperator,
        value: &Vec<String>,
    ) -> bool {
        match attribute {
            ListValueAttribute::Groups => Self::compare_groups(actor, operator, value),
            ListValueAttribute::Roles => Self::compare_roles(actor, operator, value),
        }
    }

    fn compare_groups(actor: &Actor, operator: &ListValueOperator, value: &Vec<String>) -> bool {
        match operator {
            ListValueOperator::Equal => actor.actor_groups() == value,
            ListValueOperator::Contains => Self::find_in_list(actor.actor_groups(), value),
        }
    }

    fn compare_roles(actor: &Actor, operator: &ListValueOperator, value: &Vec<String>) -> bool {
        match operator {
            ListValueOperator::Equal => actor.actor_roles() == value,
            ListValueOperator::Contains => Self::find_in_list(actor.actor_roles(), value),
        }
    }

    fn find_in_list(actor_list: &[String], to_find: &[String]) -> bool {
        for to_find_item in to_find {
            if !actor_list.contains(to_find_item) {
                return false;
            }
        }

        true
    }

    fn apply_attribute_comparation(
        actor: &Actor,
        resource: &Resource,
        left_attribute: &SingleValueAttribute,
        operator: &SingleValueOperator,
        right_attribute: &ResourceAttribute,
    ) -> bool {
        match left_attribute {
            SingleValueAttribute::Type => todo!(),
            SingleValueAttribute::Id => todo!(),
        }
    }

    //fn compare_type

    pub fn apply(&self, actor: &Actor, resource: &Resource) -> bool {
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
            Requirement::AttributesComparation {
                left_attribute,
                operator,
                right_attribute,
            } => Self::apply_attribute_comparation(
                actor,
                resource,
                left_attribute,
                operator,
                right_attribute,
            ),
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
            Token::AttributeComparationRequirement(tokens) => Ok(Self::attributes_comparation_from_tokens(tokens)),
            _ => unreachable!(),
        }
    }
}
