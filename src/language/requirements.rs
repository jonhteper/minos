use derived::Ctor;
use getset::{CopyGetters, Getters};

use crate::{
    engine::{Actor, Resource},
    errors::Error,
};

use crate::parser::tokens::{
    ActorListValueAttribute, ActorSingleValueAttribute, Indentifier, ListValueOperator,
    ResourceAttribute, SingleValueOperator, Token,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Requirement {
    SingleValue(SingleValueRequirement),
    ListValue(ListValueRequirement),
    AttributesComparation(AttributesComparationRequirement),
}

impl Requirement {
    pub fn apply(&self, actor: &Actor, resource: &Resource) -> bool {
        match self {
            Requirement::SingleValue(requirement) => requirement.apply(actor),
            Requirement::ListValue(requirement) => requirement.apply(actor),
            Requirement::AttributesComparation(requirement) => requirement.apply(actor, resource),
        }
    }
}

impl From<SingleValueRequirement> for Requirement {
    fn from(value: SingleValueRequirement) -> Self {
        Self::SingleValue(value)
    }
}

impl From<ListValueRequirement> for Requirement {
    fn from(value: ListValueRequirement) -> Self {
        Self::ListValue(value)
    }
}

impl From<AttributesComparationRequirement> for Requirement {
    fn from(value: AttributesComparationRequirement) -> Self {
        Self::AttributesComparation(value)
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
            Token::SingleValueRequirement(tokens) => {
                Ok(SingleValueRequirement::from(tokens).into())
            }
            Token::ListValueRequirement(tokens) => Ok(ListValueRequirement::from(tokens).into()),
            Token::AttributeComparationRequirement(tokens) => {
                Ok(AttributesComparationRequirement::from(tokens).into())
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Ctor, Getters, CopyGetters)]
pub struct SingleValueRequirement {
    #[getset(get_copy = "pub")]
    attribute: ActorSingleValueAttribute,

    #[getset(get_copy = "pub")]
    operator: SingleValueOperator,

    #[getset(get = "pub")]
    value: String,
}

impl SingleValueRequirement {
    fn apply(&self, actor: &Actor) -> bool {
        match self.attribute {
            ActorSingleValueAttribute::Type => self.compare_type(actor),
            ActorSingleValueAttribute::Id => self.compare_id(actor),
        }
    }

    fn compare_type(&self, actor: &Actor) -> bool {
        match self.operator {
            SingleValueOperator::Equal => actor.actor_type() == &self.value,
            SingleValueOperator::Distinct => actor.actor_type() != &self.value,
        }
    }

    fn compare_id(&self, actor: &Actor) -> bool {
        match self.operator {
            SingleValueOperator::Equal => actor.actor_id() == &self.value,
            SingleValueOperator::Distinct => actor.actor_id() != &self.value,
        }
    }
}

impl From<&Vec<Token<'_>>> for SingleValueRequirement {
    fn from(tokens: &Vec<Token>) -> Self {
        let Indentifier(value) = tokens[2].inner_identifier().unwrap();
        Self {
            attribute: tokens[0].inner_actor_single_value_attribute().unwrap(),
            operator: tokens[1].inner_single_value_operator().unwrap(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Ctor, Getters, CopyGetters)]
pub struct ListValueRequirement {
    #[getset(get_copy = "pub")]
    attribute: ActorListValueAttribute,

    #[getset(get_copy = "pub")]
    operator: ListValueOperator,

    #[getset(get = "pub")]
    value: Vec<String>,
}

impl ListValueRequirement {
    fn apply(&self, actor: &Actor) -> bool {
        match self.attribute {
            ActorListValueAttribute::Groups => self.compare_groups(actor),
            ActorListValueAttribute::Roles => self.compare_roles(actor),
        }
    }

    fn compare_groups(&self, actor: &Actor) -> bool {
        match self.operator {
            ListValueOperator::Equal => actor.actor_groups() == &self.value,
            ListValueOperator::Contains => Self::find_in_list(actor.actor_groups(), &self.value),
        }
    }

    fn compare_roles(&self, actor: &Actor) -> bool {
        match self.operator {
            ListValueOperator::Equal => actor.actor_roles() == &self.value,
            ListValueOperator::Contains => Self::find_in_list(actor.actor_roles(), &self.value),
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
}

impl From<&Vec<Token<'_>>> for ListValueRequirement {
    fn from(tokens: &Vec<Token>) -> Self {
        Self {
            attribute: tokens[0].inner_actor_list_value_attribute().unwrap(),
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

#[derive(Debug, Clone, PartialEq, Ctor, CopyGetters, Getters, Copy)]
#[getset(get_copy = "pub")]
pub struct AttributesComparationRequirement {
    left_attribute: ActorSingleValueAttribute,
    operator: SingleValueOperator,
    right_attribute: ResourceAttribute,
}

impl AttributesComparationRequirement {
    fn apply(&self, actor: &Actor, resource: &Resource) -> bool {
        match self.left_attribute {
            ActorSingleValueAttribute::Type => self.compare_actor_type(actor, resource),
            ActorSingleValueAttribute::Id => self.compare_actor_id(actor, resource),
        }
    }

    fn compare_actor_type(&self, actor: &Actor, resource: &Resource) -> bool {
        match (self.operator, self.right_attribute) {
            (SingleValueOperator::Equal, ResourceAttribute::Id) => {
                Some(actor.actor_type()) == resource.id().as_ref()
            }
            (SingleValueOperator::Equal, ResourceAttribute::Type) => {
                actor.actor_type() == resource.resource_type()
            }
            (SingleValueOperator::Distinct, ResourceAttribute::Id) => {
                Some(actor.actor_type()) != resource.id().as_ref()
            }
            (SingleValueOperator::Distinct, ResourceAttribute::Type) => {
                actor.actor_type() != resource.resource_type()
            }
        }
    }

    fn compare_actor_id(&self, actor: &Actor, resource: &Resource) -> bool {
        match (self.operator, self.right_attribute) {
            (SingleValueOperator::Equal, ResourceAttribute::Id) => {
                Some(actor.actor_id()) == resource.id().as_ref()
            }
            (SingleValueOperator::Equal, ResourceAttribute::Type) => {
                actor.actor_id() == resource.resource_type()
            }
            (SingleValueOperator::Distinct, ResourceAttribute::Id) => {
                Some(actor.actor_id()) != resource.id().as_ref()
            }
            (SingleValueOperator::Distinct, ResourceAttribute::Type) => {
                actor.actor_id() != resource.resource_type()
            }
        }
    }
}

impl From<&Vec<Token<'_>>> for AttributesComparationRequirement {
    fn from(tokens: &Vec<Token>) -> Self {
        Self {
            left_attribute: tokens[0].inner_actor_single_value_attribute().unwrap(),
            operator: tokens[1].inner_single_value_operator().unwrap(),
            right_attribute: tokens[2].inner_resource_attribute().unwrap(),
        }
    }
}
