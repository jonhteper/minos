use std::{fs, env};

use pest::Parser;

use crate::minos::file::File;
use crate::minos::lang::Token;
use crate::{minos::parser::MinosParser, errors::MinosResult};
use crate::minos::parser::v0_14::{MinosParserV0_14, Rule};

const V0_14_MINOS_CONTENT: &str = r#"
sintaxis=0.14;

/* Ejemplo de comentario*/
env ProductUseCases {
    resource Product {
        policy {
            allow = ["crear", "eliminar"];

            rule {
                actor.type = RootUser; /*Comentario2*/
            }

            rule {
                actor.groups *= ["ventas"];
                actor.roles = ["admin"];
            }

            rule {
                actor.groups *= ["ventas"];
                actor.roles = ["admin"];
            }
        }

        policy {
            allow = ["get inner"];

            rule {
                actor.type = CostumerUser;
            }

        }
    }
}

env ProductUseCases--tests {
    resource example {
        id = "id.example.a158$";

        policy {
            allow = ["crear", "obtener", "modificar", "eliminar"];

            rule {
                actor.type != RootUser;
            }
        }
    }
}
"#;


#[test]
pub fn parser_test() -> MinosResult<()> {
    let pairs = MinosParserV0_14::parse(Rule::file, &V0_14_MINOS_CONTENT)?.next().unwrap();
    let file_token = MinosParserV0_14::parse_token(pairs)?;

    match file_token {
        Token::File(_) => {},
        _ => panic!("Expect Token::File")
    }

    Ok(())

}

#[test]
fn parse_file_works() -> MinosResult<()> {
    let mut path = env::current_dir()?;
    path.push("assets/test.minos");

    let file = MinosParser::parse_file(&path)?;

    Ok(())
}
