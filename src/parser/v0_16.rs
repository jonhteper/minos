use std::str::FromStr;

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::MinosResult;

use super::tokens::{
    ActorListValueAttribute, ActorSingleValueAttribute, Array, FileVersion, Identifier,
    ListValueOperator, ResourceAttribute, SingleValueOperator, Token,
};

#[derive(Debug, Parser)]
#[grammar = "../assets/minos-v0_16.pest"]
pub struct MinosParserV0_16;

impl MinosParserV0_16 {
    fn parse_tokens(pair: Pair<Rule>) -> MinosResult<Vec<Token>> {
        pair.into_inner().map(Self::parse_token).collect()
    }

    pub(crate) fn parse_token(pair: Pair<Rule>) -> MinosResult<Token> {
        Ok(match pair.as_rule() {
            Rule::file => Token::File(Self::parse_tokens(pair)?),
            Rule::version => Token::Version(FileVersion::from_str(pair.as_str())?),
            Rule::resource => Token::Resource(Self::parse_tokens(pair)?),
            Rule::env => Token::Env(Self::parse_tokens(pair)?),
            Rule::resource_id => Token::String(pair.as_str()),
            Rule::rule => Token::Rule(Self::parse_tokens(pair)?),
            Rule::policy => Token::Policy(Self::parse_tokens(pair)?),
            Rule::allow => Token::Allow(Self::parse_tokens(pair)?),
            Rule::array => {
                let inner_values = pair.into_inner().map(|p| p.as_str()).collect();

                Token::Array(Array(inner_values))
            }
            Rule::requirement => Token::Requirement(Self::parse_tokens(pair)?),
            Rule::single_value_requirement => {
                Token::SingleValueRequirement(Self::parse_tokens(pair)?)
            }
            Rule::list_value_requirement => Token::ListValueRequirement(Self::parse_tokens(pair)?),
            Rule::attribute_comparison_requirement => {
                Token::AttributeComparisonRequirement(Self::parse_tokens(pair)?)
            }
            Rule::actor_single_value_attribute => Token::ActorSingleValueAttribute(
                ActorSingleValueAttribute::from_str(pair.as_str())?,
            ),
            Rule::single_value_operator => {
                Token::SingleValueOperator(SingleValueOperator::from_str(pair.as_str())?)
            }
            Rule::actor_list_value_attribute => {
                Token::ActorListValueAttribute(ActorListValueAttribute::from_str(pair.as_str())?)
            }
            Rule::list_value_operator => {
                Token::ListValueOperator(ListValueOperator::from_str(pair.as_str())?)
            }
            Rule::resource_attribute => {
                Token::ResourceAttribute(ResourceAttribute::from_str(pair.as_str())?)
            }
            Rule::identifier => Token::Identifier(Identifier(pair.as_str())),
            Rule::string => Token::String(pair.as_str()),
            Rule::COMMENT | Rule::char | Rule::WHITESPACE | Rule::EOI => Token::Null,
        })
    }
}
