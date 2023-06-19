use parse_display::{Display, FromStr};

#[derive(Debug, Clone, PartialEq, Eq, Display)]
pub enum Token<'a> {
    #[display("File")]
    File(Vec<Token<'a>>),

    #[display("Version")]
    Version(FileVersion),

    #[display("Env")]
    Env(Vec<Token<'a>>),

    #[display("Resource")]
    Resource(Vec<Token<'a>>),

    #[display("Rule")]
    Rule(Vec<Token<'a>>),

    #[display("Policy")]
    Policy(Vec<Token<'a>>),

    #[display("Allow")]
    Allow(Vec<Token<'a>>),

    #[display("Array")]
    Array(Array<'a>),

    #[display("Requirement")]
    Requirement(Vec<Token<'a>>),

    #[display("SingleValueRequirement")]
    SingleValueRequirement(Vec<Token<'a>>),

    #[display("ListValueRequirement")]
    ListValueRequirement(Vec<Token<'a>>),

    #[display("AttributeComparationRequirement")]
    AttributeComparationRequirement(Vec<Token<'a>>),

    #[display("ResourceAttribute")]
    ResourceAttribute(ResourceAttribute),

    #[display("SingleValueAttribute")]
    SingleValueAttribute(SingleValueAttribute),

    #[display("SingleValueOperator")]
    SingleValueOperator(SingleValueOperator),

    #[display("ListValueAttribute")]
    ListValueAttribute(ListValueAttribute),

    #[display("ListValueOperator")]
    ListValueOperator(ListValueOperator),

    #[display("Identifier")]
    Identifier(Indentifier<'a>),

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
        if let Token::Env(inner) = self {
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

    pub fn inner_single_value_requirement(&self) -> Option<&Vec<Token<'a>>> {
        if let Token::SingleValueRequirement(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn inner_list_value_requirement(&self) -> Option<&Vec<Token<'a>>> {
        if let Token::ListValueRequirement(inner) = self {
            return Some(inner);
        }

        None
    }

    pub fn attribute_comparation_requirement(&self) -> Option<&Vec<Token<'a>>> {
        if let Token::AttributeComparationRequirement(inner) = self {
            return Some(inner);
        }

        None
    }
    pub fn resource_attribute(&self) -> Option<ResourceAttribute> {
        if let Token::ResourceAttribute(inner) = self {
            return Some(*inner);
        }

        None
    }

    pub fn inner_single_value_attribute(&self) -> Option<SingleValueAttribute> {
        if let Token::SingleValueAttribute(inner) = self {
            return Some(*inner);
        }

        None
    }

    pub fn inner_single_value_operator(&self) -> Option<SingleValueOperator> {
        if let Token::SingleValueOperator(inner) = self {
            return Some(*inner);
        }

        None
    }

    pub fn inner_list_value_attribute(&self) -> Option<ListValueAttribute> {
        if let Token::ListValueAttribute(inner) = self {
            return Some(*inner);
        }

        None
    }

    pub fn inner_list_value_operator(&self) -> Option<ListValueOperator> {
        if let Token::ListValueOperator(inner) = self {
            return Some(*inner);
        }

        None
    }

    pub fn inner_identifier(&self) -> Option<Indentifier<'a>> {
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
    #[display("0.14")]
    V0_14,
    #[display("0.15")]
    V0_15,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array<'a>(pub Vec<&'a str>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Indentifier<'a>(pub &'a str);

#[derive(Debug, Clone, Copy, Display, FromStr, PartialEq, Eq)]
pub enum SingleValueAttribute {
    #[display("actor.type")]
    Type,

    #[display("actor.id")]
    Id,
}

#[derive(Debug, Clone, Copy, Display, FromStr, PartialEq, Eq)]
pub enum ListValueAttribute {
    #[display("actor.groups")]
    Groups,

    #[display("actor.roles")]
    Roles,
}

#[derive(Debug, Clone, Copy, Display, FromStr, PartialEq, Eq)]
pub enum SingleValueOperator {
    #[display("=")]
    Equal,

    #[display("!=")]
    Distinct,
}

#[derive(Debug, Clone, Copy, Display, FromStr, PartialEq, Eq)]
pub enum ListValueOperator {
    #[display("=")]
    Equal,

    #[display("*=")]
    Contains,
}

#[derive(Debug, Clone, Copy, Display, FromStr, PartialEq, Eq)]
pub enum ResourceAttribute {
    #[display("resource.id")]
    Id,

    #[display("resource.type")]
    Type,
}

