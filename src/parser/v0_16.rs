use std::{collections::HashMap, str::FromStr, sync::Arc};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{language::storage::Storage, Error, MinosResult};

use super::tokens::{
    ActorAttribute, Array, FileVersion, Identifier, Operator, ResourceAttribute, Token,
};

#[derive(Debug, Parser)]
#[grammar = "../assets/minos-v0_16.pest"]
pub(crate) struct MinosParserV0_16;

impl MinosParserV0_16 {
    fn parse_tokens(
        pair: Pair<Rule>,
        values_map: &mut HashMap<String, Arc<str>>,
    ) -> MinosResult<Vec<Token>> {
        pair.into_inner()
            .map(|p| Self::parse_token(p, values_map))
            .collect()
    }

    fn extract_next_str(pair: Pair<Rule>) -> Option<&str> {
        pair.into_inner().next().map(|inner_pair| inner_pair.as_str())
    }

    fn extract_next_array(
        pair: Pair<Rule>,
        values_map: &mut HashMap<String, Arc<str>>,
    ) -> Vec<Arc<str>> {
        pair.into_inner()
            .flat_map(|pair| match Self::parse_token(pair, values_map).unwrap() {
                Token::String(value) => Some(value),
                _ => None,
            })
            .collect()
    }

    fn get_optimized_pointer(values_map: &mut HashMap<String, Arc<str>>, value: &str) -> Arc<str> {
        match values_map.get(value) {
            Some(val) => val.clone(),
            None => {
                let arc: Arc<str> = Arc::from(value);
                values_map.insert(value.to_string(), arc.clone());

                arc
            }
        }
    }

    pub(crate) fn parse_token(
        pair: Pair<Rule>,
        values_map: &mut HashMap<String, Arc<str>>,
    ) -> MinosResult<Token> {
        let token = match pair.as_rule() {
            Rule::file => Token::File(Self::parse_tokens(pair, values_map)?),
            Rule::version => Token::Version(FileVersion::from_str(pair.as_str())?),
            Rule::resource => Token::Resource(Self::parse_tokens(pair, values_map)?),
            Rule::attributed_resource => {
                Token::AttributedResource(Self::parse_tokens(pair, values_map)?)
            }
            Rule::named_env => Token::NamedEnv(Self::parse_tokens(pair, values_map)?),
            Rule::default_env => Token::DefaultEnv(Self::parse_tokens(pair, values_map)?),
            Rule::implicit_default_env => {
                Token::ImplicitDefaultEnv(Self::parse_tokens(pair, values_map)?)
            }
            Rule::policy => Token::Policy(Self::parse_tokens(pair, values_map)?),
            Rule::allow => Token::Allow(Self::parse_tokens(pair, values_map)?),
            Rule::rule => Token::Rule(Self::parse_tokens(pair, values_map)?),
            Rule::array => {
                let inner_values = Self::extract_next_array(pair, values_map);
                Token::Array(Array(inner_values))
            }
            Rule::requirement => Token::Requirement(Self::parse_tokens(pair, values_map)?),
            Rule::assertion => Token::Assertion(Self::parse_tokens(pair, values_map)?),
            Rule::negation => Token::Negation(Self::parse_tokens(pair, values_map)?),
            Rule::search => Token::Search(Self::parse_tokens(pair, values_map)?),
            Rule::actor_id => Token::ActorAttribute(ActorAttribute::Id),
            Rule::actor_type => Token::ActorAttribute(ActorAttribute::Type),
            Rule::actor_groups => Token::ActorAttribute(ActorAttribute::Groups),
            Rule::actor_roles => Token::ActorAttribute(ActorAttribute::Roles),
            Rule::actor_status => Token::ActorAttribute(ActorAttribute::Status),
            Rule::resource_id => Token::ResourceAttribute(ResourceAttribute::Id),
            Rule::resource_type => Token::ResourceAttribute(ResourceAttribute::Type),
            Rule::resource_owner => Token::ResourceAttribute(ResourceAttribute::Owner),
            Rule::resource_status => Token::ResourceAttribute(ResourceAttribute::Status),
            Rule::assertion_operator => Token::Operator(Operator::Assertion),
            Rule::negation_operator => Token::Operator(Operator::Negation),
            Rule::search_operator => Token::Operator(Operator::Search),
            Rule::identifier => {
                let val = Self::get_optimized_pointer(values_map, pair.as_str());
                Token::Identifier(Identifier(val))
            }
            Rule::string => {
                let value = Self::extract_next_str(pair).ok_or(Error::MissingToken)?;
                let val = Self::get_optimized_pointer(values_map, value);
                Token::String(val)
            }
            Rule::inner_string | Rule::COMMENT | Rule::char | Rule::WHITESPACE | Rule::EOI => {
                Token::Null
            }
        };

        Ok(token)
    }

    pub fn parse_file_content(
        content: &str,
        values_map: &mut HashMap<String, Arc<str>>,
    ) -> MinosResult<Storage> {
        let file_rules = Self::parse(Rule::file, content)?.next().unwrap();
        let file_token = Self::parse_token(file_rules, values_map)?;
        let storage = Storage::try_from(file_token)?;

        Ok(storage)
    }
}
