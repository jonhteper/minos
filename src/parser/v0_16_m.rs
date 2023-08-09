use std::{collections::HashMap, str::FromStr, sync::Arc};

use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use crate::{language::storage::Storage, Error, MinosResult};

use super::tokens::{
    ActorAttribute, Array, FileVersion, Identifier, Operator, ResourceAttribute, Token,
};

#[derive(Debug, Parser)]
#[grammar = "../assets/minos-v0_16M.pest"]
pub(crate) struct MinosParserV0_16M;

impl MinosParserV0_16M {
    fn parse_tokens(
        pair: Pair<Rule>,
        macro_tokens: &mut HashMap<Identifier, Vec<Token>>,
        values_map: &mut HashMap<String, Arc<str>>,
    ) -> MinosResult<Vec<Token>> {
        pair.into_inner()
            .map(|p| Self::parse_token(p, macro_tokens, values_map))
            .collect()
    }

    fn get_optimized_pointer(
        values_map: &mut HashMap<String, Arc<str>>,
        value: &str,
    ) -> Arc<str> {
        match values_map.get(value) {
            Some(val) => val.clone(),
            None => {
                let arc: Arc<str> = Arc::from(value);
                values_map.insert(value.to_string(), arc.clone());

                arc
            }
        }
    }

    fn extract_next_str(pair: Pair<Rule>) -> Option<&str> {
        pair.into_inner().next().map(|inner_pair| inner_pair.as_str())
    }

    fn extract_next_identifier(
        pair: Pair<Rule>,
        values_map: &mut HashMap<String, Arc<str>>,
    ) -> Option<Identifier> {
        pair.into_inner()
            .next()
            .map(|rule| {
                let arc_val = Self::get_optimized_pointer(values_map, rule.as_str());
                Identifier(arc_val)
            })
    }

    fn parse_array(
        pair: Pair<Rule>,
        macro_tokens: &mut HashMap<Identifier, Vec<Token>>,
        values_map: &mut HashMap<String, Arc<str>>,
    ) -> MinosResult<Vec<Arc<str>>> {
        let mut permissions = vec![];
        for pair in pair.into_inner() {
            let parsed_token = Self::parse_token(pair, macro_tokens, values_map)?;
            match parsed_token {
                Token::String(permission) => permissions.push(permission),
                Token::MacroCall(tokens) => {
                    for token in tokens {
                        if let Token::String(permission) = token {
                            permissions.push(permission);
                        }
                    }
                }
                _ => Err(Error::InvalidToken {
                    expected: "String",
                    found: parsed_token.to_string(),
                })?,
            }
        }

        Ok(permissions)
    }

    fn extract_macro_tokens(
        pair: Pair<Rule>,
        macro_tokens: &mut HashMap<Identifier, Vec<Token>>,
        values_map: &mut HashMap<String, Arc<str>>,
    ) -> MinosResult<()> {
        let mut inner_tokens = Self::parse_tokens(pair, macro_tokens, values_map)?;
        let first_token = inner_tokens[0].clone();
        let ident = match first_token {
            Token::Identifier(ident) => ident,
            _ => Err(Error::InvalidToken {
                expected: "Identifier",
                found: first_token.to_string(),
            })?,
        };
        let _ = inner_tokens.remove(0);
        macro_tokens.insert(ident, inner_tokens);

        Ok(())
    }

    fn extract_requirements(
        pair: Pair<Rule>,
        macro_tokens: &mut HashMap<Identifier, Vec<Token>>,
        values_map: &mut HashMap<String, Arc<str>>,
    ) -> MinosResult<Vec<Token>> {
        let mut requirements = vec![];
        for p in pair.into_inner() {
            let parsed_token = Self::parse_token(p, macro_tokens, values_map)?;
            match parsed_token {
                Token::MacroCall(mut tokens) => requirements.append(&mut tokens),
                Token::Requirement(_) => requirements.push(parsed_token),
                _ => Err(Error::InvalidToken {
                    expected: "MacroCall or Requirement",
                    found: parsed_token.to_string(),
                })?,
            }
        }

        Ok(requirements)
    }

    pub(crate) fn parse_token(
        pair: Pair<Rule>,
        macro_tokens: &mut HashMap<Identifier, Vec<Token>>,
        values_map: &mut HashMap<String, Arc<str>>,
    ) -> MinosResult<Token> {
        let token = match pair.as_rule() {
            Rule::file => Token::File(Self::parse_tokens(pair, macro_tokens, values_map)?),
            Rule::version => Token::Version(FileVersion::from_str(pair.as_str())?),
            Rule::macro_definition => {
                Self::extract_macro_tokens(pair, macro_tokens, values_map)?;
                Token::MacroDefinition
            }
            Rule::macro_call => {
                let macro_ident =
                    Self::extract_next_identifier(pair, values_map).ok_or(Error::MissingToken)?;
                let macro_tokens = macro_tokens
                    .get(&macro_ident)
                    .ok_or(Error::MacroNotExist(macro_ident.0.to_string()))?;
                Token::MacroCall(macro_tokens.clone())
            }
            Rule::resource => Token::Resource(Self::parse_tokens(pair, macro_tokens, values_map)?),
            Rule::attributed_resource => {
                Token::AttributedResource(Self::parse_tokens(pair, macro_tokens, values_map)?)
            }
            Rule::named_env => Token::NamedEnv(Self::parse_tokens(pair, macro_tokens, values_map)?),
            Rule::default_env => Token::DefaultEnv(Self::parse_tokens(pair, macro_tokens, values_map)?),
            Rule::implicit_default_env => {
                Token::ImplicitDefaultEnv(Self::parse_tokens(pair, macro_tokens, values_map)?)
            }
            Rule::policy => Token::Policy(Self::parse_tokens(pair, macro_tokens, values_map)?),
            Rule::allow => Token::Allow(Self::parse_tokens(pair, macro_tokens, values_map)?),
            Rule::rule => {
                let requirements = Self::extract_requirements(pair, macro_tokens, values_map)?;

                Token::Rule(requirements)
            }
            Rule::array => {
                let permissions = Self::parse_array(pair, macro_tokens, values_map)?;
                Token::Array(Array(permissions))
            }
            Rule::requirement => Token::Requirement(Self::parse_tokens(pair, macro_tokens, values_map)?),
            Rule::assertion => Token::Assertion(Self::parse_tokens(pair, macro_tokens, values_map)?),
            Rule::negation => Token::Negation(Self::parse_tokens(pair, macro_tokens, values_map)?),
            Rule::search => Token::Search(Self::parse_tokens(pair, macro_tokens, values_map)?),
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
                let arc_val = Self::get_optimized_pointer(values_map, pair.as_str());
                Token::Identifier(Identifier(arc_val))
            }
            Rule::string => {
                let inner_str = Self::extract_next_str(pair).ok_or(Error::MissingToken)?;
                let arc_val = Self::get_optimized_pointer(values_map, inner_str);
                Token::String(arc_val)
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
        let mut macro_tokens: HashMap<Identifier, Vec<Token>> = HashMap::new();
        let file_token = Self::parse_token(file_rules, &mut macro_tokens, values_map)?;
        let storage = Storage::try_from(file_token)?;

        Ok(storage)
    }
}
