use crate::errors::Error;

use super::lang::{
    ListValueAttribute, ListValueOperator, SingleValueAttribute, SingleValueOperator, Token,
};

#[derive(Debug, Clone)]
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
        Self::SingleValue {
            attribute: tokens[0].inner_single_value_attribute().unwrap(),
            operator: tokens[1].inner_single_value_operator().unwrap(),
            value: tokens[2].inner_string().unwrap().to_string(),
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

/*
#[derive(Debug, Clone, Ctor, Getters)]
#[getset(get = "pub")]
pub struct Requirement {
    attribute: ActorAttribute,
    operator: Operator,
    value: Value,
}

impl Requirement {
    fn compare_type(&self, actor: &impl Actor) -> Result<bool, Error> {
        match self.operator {
            Operator::Equal => Ok(&actor.actor_type() == self.value.try_as_str()?),
            Operator::Distinct => Ok(&actor.actor_type() != self.value.try_as_str()?),
            Operator::Contains => Err(Error::InvalidOperation(Operator::Contains)),
        }
    }

    fn compare_id(&self, actor: &impl Actor) -> Result<bool, Error> {
        match self.operator {
            Operator::Equal => Ok(&actor.actor_id() == self.value.try_as_str()?),
            Operator::Distinct => Ok(&actor.actor_id() != self.value.try_as_str()?),
            Operator::Contains => Err(Error::InvalidOperation(Operator::Contains)),
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

    fn compare_groups(&self, actor: &impl Actor) -> Result<bool, Error> {
        match self.operator {
            Operator::Equal => Ok(self.value.try_as_list()? == &actor.actor_groups()),
            Operator::Distinct => Err(Error::InvalidOperation(Operator::Distinct)),
            Operator::Contains => Ok(Self::find_in_list(
                &actor.actor_groups(),
                self.value.try_as_list()?,
            )),
        }
    }

    fn compare_roles(&self, actor: &impl Actor) -> Result<bool, Error> {
        match self.operator {
            Operator::Equal => Ok(self.value.try_as_list()? == &actor.actor_roles()),
            Operator::Distinct => Err(Error::InvalidOperation(Operator::Distinct)),
            Operator::Contains => Ok(Self::find_in_list(
                &actor.actor_roles(),
                self.value.try_as_list()?,
            )),
        }
    }

    pub fn apply(&self, actor: &impl Actor) -> Result<bool, Error> {
        match self.attribute {
            ActorAttribute::Type => self.compare_type(actor),
            ActorAttribute::Id => self.compare_id(actor),
            ActorAttribute::Groups => self.compare_groups(actor),
            ActorAttribute::Roles => self.compare_roles(actor),
        }
    }
}


#[derive(Debug, Clone, Copy, Display, FromStr)]
pub enum ActorAttribute {
    #[display("actor.type")]
    Type,

    #[display("actor.id")]
    Id,

    #[display("actor.groups")]
    Groups,

    #[display("actor.roles")]
    Roles,
}

#[derive(Debug, Clone, Copy, Display, FromStr, PartialEq, Eq)]
pub enum Operator {
    #[display("=")]
    Equal,

    #[display("*=")]
    Contains,

    #[display("!=")]
    Distinct,
}

#[derive(Debug, Clone)]
pub enum Value {
    Str(String),
    List(Vec<String>),
}

impl Value {
    pub fn try_as_str(&self) -> Result<&String, Error> {
        match self {
            Value::Str(val) => Ok(val),
            Value::List(_) => Err(Error::UnwrapInvalidStringValue),
        }
    }

    pub fn try_as_list(&self) -> Result<&Vec<String>, Error> {
        match self {
            Value::List(val) => Ok(val),
            Value::Str(_) => Err(Error::UnwrapInvalidListValue),
        }
    }
}



*/
