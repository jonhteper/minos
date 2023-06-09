use std::collections::HashMap;

use derived::Ctor;
use getset::Getters;
use parse_display::{Display, FromStr};
use versions::Version;

use crate::{errors::Error, authorization::Actor};


#[derive(Debug, Clone, Ctor, Getters)]
#[getset(get = "pub")]
pub struct File {
    sintaxis_version: Version,
    environments: HashMap<EnvName, Environment>,
}

pub type EnvName = String;

#[derive(Debug, Clone, Ctor, Getters)]
#[getset(get = "pub")]
pub struct Environment {
    name: EnvName,
    resources: HashMap<(ResourceName, Option<ResourceId>), Resource>,
}

pub type ResourceId = String;
pub type ResourceName = String;

#[derive(Debug, Clone, Ctor, Getters)]
#[getset(get = "pub")]
pub struct Resource {
    name: ResourceName,
    id: Option<ResourceId>,
    policies: Vec<Policy>,
}

#[derive(Debug, Clone, Ctor, Getters)]
#[getset(get = "pub")]
pub struct Policy {
    allow: Vec<Permission>,
    rules: Vec<AuthorizationRule>,
}

impl Policy {
    /// Returns the [Permission] list f the actor satisfies at least one of the rules.
    pub fn apply(&self, actor: &impl Actor) -> Result<&Vec<Permission>, Error> {
        for rule in &self.rules {
            if rule.apply(actor)? {
                return Ok(&self.allow);
            }
        }
        
        Err(Error::ActorNotAuthorized(actor.actor_id()))
    }
}


pub type Permission = String;

#[derive(Debug, Clone, Ctor, Getters)]
#[getset(get = "pub")]
pub struct AuthorizationRule {
    requirements: Vec<Requirement>,
}

impl AuthorizationRule {
    /// Apply all requirements and return true only if actor satisfies all.
    pub fn apply(&self, actor: &impl Actor) -> Result<bool, Error> {
        let mut is_authorized = false;

        for requirement in &self.requirements {
            is_authorized = requirement.apply(actor)?;
        }

        Ok(is_authorized)
    }
}


#[derive(Debug, Clone, Ctor, Getters)]
#[getset(get = "pub")]
pub struct Requirement {
    attribute: ActorAttribute,
    operator: Operator,
    value: Value,
}

impl Requirement {
    fn compare_type(&self, actor: &impl Actor) -> Result<bool, Error>{
        match self.operator {
            Operator::Equal => Ok(&actor.actor_type() == self.value.try_as_str()?),
            Operator::Distinct => todo!(),
            Operator::Contains => todo!(),
        }
    }

    fn compare_id(&self, actor: &impl Actor) -> Result<bool, Error>{
        match self.operator {
            Operator::Equal => todo!(),
            Operator::Distinct => todo!(),
            Operator::Contains => todo!(),
        }
    }

    fn compare_groups(&self, actor: &impl Actor) -> Result<bool, Error>{
        match self.operator {
            Operator::Equal => todo!(),
            Operator::Distinct => todo!(),
            Operator::Contains => todo!(),
        }
    }

    fn compare_roles(&self, actor: &impl Actor) -> Result<bool, Error>{
        match self.operator {
            Operator::Equal => todo!(),
            Operator::Distinct => todo!(),
            Operator::Contains => todo!(),
        }
    }


    pub fn apply(&self, actor: &impl Actor) -> Result<bool, Error>{
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
    #[display("actor.tyoe")]
    Type,

    #[display("actor.id")]
    Id,

    #[display("actor.groups")]
    Groups,

    #[display("actor.roles")]
    Roles,
}

#[derive(Debug, Clone, Copy, Display, FromStr)]
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