use pest::{iterators::Pair, Parser};
use pest_derive::Parser;
use std::collections::HashMap;
use std::str::FromStr;

use crate::errors::MinosResult;
use crate::language::environment::{EnvName, Environment};
use crate::language::file::File;
use crate::parser::tokens::{self, ActorListValueAttribute, Array, Identifier};

#[derive(Debug, Parser)]
#[grammar = "../assets/minos.pest"]
pub struct MinosParserV0_14;

impl MinosParserV0_14 {
    fn parse_tokens(pair: Pair<Rule>) -> MinosResult<Vec<tokens::Token>> {
        pair.into_inner().map(Self::parse_token).collect()
    }

    pub(crate) fn parse_token(pair: Pair<Rule>) -> MinosResult<tokens::Token> {
        use tokens::Token;

        Ok(match pair.as_rule() {
            Rule::file => Token::File(Self::parse_tokens(pair)?),
            Rule::version => Token::Version(tokens::FileVersion::from_str(pair.as_str())?),
            Rule::env => Token::Env(Self::parse_tokens(pair)?),
            Rule::resource => Token::Resource(Self::parse_tokens(pair)?),
            Rule::rule => Token::Rule(Self::parse_tokens(pair)?),
            Rule::policy => Token::Policy(Self::parse_tokens(pair)?),
            Rule::allow => Token::Allow(Self::parse_tokens(pair)?),
            Rule::requirement => Token::Requirement(Self::parse_tokens(pair)?),
            Rule::single_value_requirement => {
                Token::SingleValueRequirement(Self::parse_tokens(pair)?)
            }
            Rule::list_value_attribute => {
                Token::ActorListValueAttribute(ActorListValueAttribute::from_str(pair.as_str())?)
            }
            Rule::array => {
                let inner_values = pair.into_inner().map(|p| p.as_str()).collect();

                Token::Array(Array(inner_values))
            }
            Rule::identifier => Token::Identifier(Identifier(pair.as_str())),
            Rule::string => Token::String(pair.as_str()),
            Rule::resource_id => Token::String(pair.as_str()),
            Rule::single_value_attribute => Token::ActorSingleValueAttribute(
                tokens::ActorSingleValueAttribute::from_str(pair.as_str())?,
            ),
            Rule::single_value_operator => {
                Token::SingleValueOperator(tokens::SingleValueOperator::from_str(pair.as_str())?)
            }
            Rule::list_value_operator => {
                Token::ListValueOperator(tokens::ListValueOperator::from_str(pair.as_str())?)
            }
            Rule::list_value_requirement => {
                let inner_tokens: MinosResult<Vec<Token>> =
                    pair.into_inner().map(Self::parse_token).collect();
                Token::ListValueRequirement(inner_tokens?)
            }
            Rule::COMMENT | Rule::char | Rule::WHITESPACE | Rule::EOI => Token::Null,
        })
    }

    pub fn parse_file_content(content: &str) -> MinosResult<HashMap<EnvName, Environment>> {
        let file_rules = MinosParserV0_14::parse(Rule::file, content)?
            .next()
            .unwrap();
        let file_token = MinosParserV0_14::parse_token(file_rules)?;

        Ok(File::try_from(file_token)?.environments().clone())
    }
}
