use std::str::FromStr;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{
    language::{file::File, storage::Storage},
    MinosResult,
};

use super::tokens::{ActorAttribute, Array, FileVersion, Identifier, Token, ResourceAttribute, Operator};

#[derive(Debug, Parser)]
#[grammar = "../assets/minos-v0_16.pest"]
pub struct MinosParserV0_16;

impl MinosParserV0_16 {
    fn parse_tokens(pair: Pair<Rule>) -> MinosResult<Vec<Token>> {
        pair.into_inner().map(Self::parse_token).collect()
    }

    pub(crate) fn parse_token(pair: Pair<Rule>) -> MinosResult<Token> {
        let token = match pair.as_rule() {
            Rule::file => Token::File(Self::parse_tokens(pair)?),
            Rule::version => Token::Version(FileVersion::from_str(pair.as_str())?),
            Rule::resource => Token::Resource(Self::parse_tokens(pair)?),
            Rule::attributed_resource => Token::AttributedResource(Self::parse_tokens(pair)?),
            Rule::named_env => Token::NamedEnv(Self::parse_tokens(pair)?),
            Rule::default_env => Token::DefaultEnv(Self::parse_tokens(pair)?),
            Rule::implicit_default_env => Token::ImplicitDefaultEnv(Self::parse_tokens(pair)?),
            Rule::policy => Token::Policy(Self::parse_tokens(pair)?),
            Rule::allow => Token::Allow(Self::parse_tokens(pair)?),
            Rule::rule => Token::Rule(Self::parse_tokens(pair)?),
            Rule::array => {
                let inner_values: Vec<&str> = pair
                    .into_inner()
                    .flat_map(|pair| match Self::parse_token(pair).unwrap() {
                        Token::String(value) => Some(value),
                        _ => None,
                    })
                    .collect();

                Token::Array(Array(inner_values))
            }
            Rule::requirement => Token::Requirement(Self::parse_tokens(pair)?),
            Rule::assertion => Token::Assertion(Self::parse_tokens(pair)?),
            Rule::negation => Token::Negation(Self::parse_tokens(pair)?),
            Rule::search => Token::Search(Self::parse_tokens(pair)?),
            Rule::actor_id => Token::ActorAttribute(ActorAttribute::Id),
            Rule::actor_type => Token::ActorAttribute(ActorAttribute::Type),
            Rule::actor_groups => Token::ActorAttribute(ActorAttribute::Groups),
            Rule::actor_roles => Token::ActorAttribute(ActorAttribute::Roles),
            Rule::resource_id => Token::ResourceAttribute(ResourceAttribute::Id),
            Rule::resource_type => Token::ResourceAttribute(ResourceAttribute::Type),
            Rule::resource_owner => Token::ResourceAttribute(ResourceAttribute::Owner),
            Rule::assertion_operator => Token::Operator(Operator::Assertion),
            Rule::negation_operator => Token::Operator(Operator::Negation),
            Rule::search_operator => Token::Operator(Operator::Search),
            Rule::identifier => Token::Identifier(Identifier(pair.as_str())),
            Rule::string => Token::StringDefinition(Self::parse_tokens(pair)?),
            Rule::inner_string => Token::String(pair.as_str()),
            Rule::COMMENT | Rule::char | Rule::WHITESPACE | Rule::EOI => Token::Null,
        };

        Ok(token)
    }

    pub fn parse_file_content(content: &str) -> MinosResult<Storage> {
        let file_rules = Self::parse(Rule::file, content)?.next().unwrap();
        let file_token = Self::parse_token(file_rules)?;
        let storage = File::try_from(file_token)?.storage();
        Ok(*storage)
    }
}
