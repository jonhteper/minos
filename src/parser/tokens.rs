use std::sync::Arc;

use parse_display::{Display, FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum Token {
    #[display("File")]
    File(Vec<Token>),

    #[display("Version")]
    Version(FileVersion),

    #[display("Resource")]
    Resource(Vec<Token>),

    #[display("AttributedResource")]
    AttributedResource(Vec<Token>),

    #[display("NamedEnv")]
    NamedEnv(Vec<Token>),

    #[display("DefaultEnv")]
    DefaultEnv(Vec<Token>),

    #[display("ImplicitDefaultEnv")]
    ImplicitDefaultEnv(Vec<Token>),

    #[display("Policy")]
    Policy(Vec<Token>),

    #[display("Allow")]
    Allow(Vec<Token>),

    #[display("Rule")]
    Rule(Vec<Token>),

    #[display("Array")]
    Array(Array),

    #[display("Requirement")]
    Requirement(Vec<Token>),

    #[display("Assertion")]
    Assertion(Vec<Token>),

    #[display("Negation")]
    Negation(Vec<Token>),

    #[display("Search")]
    Search(Vec<Token>),

    #[display("ActorAttribute")]
    ActorAttribute(ActorAttribute),

    #[display("ResourceAttribute")]
    ResourceAttribute(ResourceAttribute),

    #[display("Operator")]
    Operator(Operator),

    #[display("StringDefinition")]
    StringDefinition(Vec<Token>),

    #[display("Identifier")]
    Identifier(Identifier),

    #[display("String")]
    String(Arc<str>),

    #[display("Null")]
    Null,
}

impl Token {
    pub fn inner_file(&self) -> Option<&Vec<Token>> {
        if let Token::File(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_version(&self) -> Option<FileVersion> {
        if let Token::Version(inner) = self {
            return Some(*inner);
        }

        None
    }

    pub fn inner_env(&self) -> Option<&Vec<Token>> {
        if let Token::NamedEnv(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_resource(&self) -> Option<&Vec<Token>> {
        if let Token::Resource(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_rule(&self) -> Option<&Vec<Token>> {
        if let Token::Rule(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_policy(&self) -> Option<&Vec<Token>> {
        if let Token::Policy(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_allow(&self) -> Option<&Vec<Token>> {
        if let Token::Allow(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_array(&self) -> Option<&Array> {
        if let Token::Array(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_requirement(&self) -> Option<&Vec<Token>> {
        if let Token::Requirement(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_resource_attribute(&self) -> Option<ResourceAttribute> {
        if let Token::ResourceAttribute(inner) = self {
            return Some(*inner);
        }

        None
    }

    pub fn inner_identifier(&self) -> Option<&Identifier> {
        if let Token::Identifier(inner) = self {
            return Some(inner);
        }

        None
    }

}

#[derive(Debug, Clone, Copy, Display, FromStr, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileVersion {
    #[display("0.16")]
    V0_16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array(pub Vec<Arc<str>>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier(pub Arc<str>);

#[derive(Debug, Clone, Copy, Display, FromStr, PartialEq, Eq)]
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
pub enum ResourceAttribute {
    #[display("resource.id")]
    Id,

    #[display("resource.type")]
    Type,

    #[display("resource.owner")]
    Owner,
}

#[derive(Debug, Clone, Copy, Display, FromStr, PartialEq, Eq)]
pub enum Operator {
    #[display("=")]
    Assertion,

    #[display("!=")]
    Negation,

    #[display("*=")]
    Search,
}
