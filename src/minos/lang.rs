use std::collections::HashMap;

use derived::Ctor;
use getset::Getters;
use parse_display::{Display, FromStr};
use pest::iterators::Pair;

use crate::{authorization::Actor, errors::Error};

#[derive(Debug, Clone)]
pub enum Token<'a> {
    File(Vec<Token<'a>>),
    Version(FileVersion),
    Env(Vec<Token<'a>>),
    Resource(Vec<Token<'a>>),
    Rule(Vec<Token<'a>>),
    Policy(Vec<Token<'a>>),
    Allow(Vec<Token<'a>>),
    Array(Array<'a>),
    Requirement(Vec<Token<'a>>),
    SingleValueRequirement(Vec<Token<'a>>),
    ListValueRequirement(Vec<Token<'a>>),
    SingleValueAttribute(SingleValueAttribute),
    SingleValueOperator(SingleValueOperator),
    ListValueAttribute(ListValueAttribute),
    ListValueOperator(ListValueOperator),
    Identifier(Indentifier<'a>),
    String(&'a str),
    Null,
}

#[derive(Debug, Clone, Copy, Display, FromStr, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileVersion {
    #[display("0.14")]
    V0_14,
}


#[derive(Debug, Clone)]
pub struct Array<'a>(pub Vec<&'a str>);

#[derive(Debug, Clone)]
pub struct Indentifier<'a>(pub &'a str);

#[derive(Debug, Clone, Copy, Display, FromStr)]
pub enum SingleValueAttribute {
    #[display("actor.type")]
    Type,

    #[display("actor.id")]
    Id,
}

#[derive(Debug, Clone, Copy, Display, FromStr)]
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
