use super::lang::{SingleValueAttribute, SingleValueOperator, ListValueAttribute, ListValueOperator};


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