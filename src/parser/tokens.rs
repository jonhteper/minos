use parse_display::{Display, FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum Token<'a> {
    #[display("File")]
    File(Vec<Token<'a>>),

    #[display("Version")]
    Version(FileVersion),

    #[display("Resource")]
    Resource(Vec<Token<'a>>),

    #[display("AttributedResource")]
    AttributedResource(Vec<Token<'a>>),

    #[display("NamedEnv")]
    NamedEnv(Vec<Token<'a>>),

    #[display("DefaultEnv")]
    DefaultEnv(Vec<Token<'a>>),

    #[display("ImplicitDefaultEnv")]
    ImplicitDefaultEnv(Vec<Token<'a>>),

    #[display("Policy")]
    Policy(Vec<Token<'a>>),

    #[display("Allow")]
    Allow(Vec<Token<'a>>),

    #[display("Rule")]
    Rule(Vec<Token<'a>>),

    #[display("Array")]
    Array(Array<'a>),

    #[display("Requirement")]
    Requirement(Vec<Token<'a>>),

    #[display("Assertion")]
    Assertion(Vec<Token<'a>>),

    #[display("Negation")]
    Negation(Vec<Token<'a>>),

    #[display("Search")]
    Search(Vec<Token<'a>>),

    #[display("ActorAttribute")]
    ActorAttribute(ActorAttribute),

    #[display("ResourceAttribute")]
    ResourceAttribute(ResourceAttribute),

    #[display("Operator")]
    Operator(Operator),

    #[display("StringDefinition")]
    StringDefinition(Vec<Token<'a>>),

    #[display("Identifier")]
    Identifier(Identifier<'a>),

    #[display("String")]
    String(&'a str),

    #[display("Null")]
    Null,
}

impl<'a> Token<'a> {
    pub fn inner_file(&self) -> Option<&Vec<Token<'a>>> {
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

    pub fn inner_env(&self) -> Option<&Vec<Token<'a>>> {
        if let Token::NamedEnv(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_resource(&self) -> Option<&Vec<Token<'a>>> {
        if let Token::Resource(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_rule(&self) -> Option<&Vec<Token<'a>>> {
        if let Token::Rule(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_policy(&self) -> Option<&Vec<Token<'a>>> {
        if let Token::Policy(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_allow(&self) -> Option<&Vec<Token<'a>>> {
        if let Token::Allow(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_array(&self) -> Option<&Array<'a>> {
        if let Token::Array(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_requirement(&self) -> Option<&Vec<Token<'a>>> {
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

    pub fn inner_identifier(&self) -> Option<Identifier<'a>> {
        if let Token::Identifier(inner) = self {
            return Some(*inner);
        }

        None
    }

    pub fn inner_string(&self) -> Option<&'a str> {
        if let Token::String(inner) = self {
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
pub struct Array<'a>(pub Vec<&'a str>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Identifier<'a>(pub &'a str);

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
