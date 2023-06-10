use std::fs;

use pest::Parser;

use crate::minos::parser::{MinosParser, Rule};

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
pub fn parser_test() {
    let pairs = MinosParser::parse(Rule::file, &V0_14_MINOS_CONTENT)
        .expect("Error getting file");
    
        for pair in pairs {
            // A pair is a combination of the rule which matched and a span of input
           
            for inner_pair in pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::EOI => println!("Fin del documento"),
                    Rule::file => println!("Documento válido"),
                    Rule::env => println!("Env: {}", inner_pair.as_str()),
                    Rule::resource => println!("Resource: {}", inner_pair.as_str()),
                    Rule::rule => println!("Resource: {}", inner_pair.as_str()),
                    Rule::policy => println!("Policy: {}", inner_pair.as_str()),
                    Rule::actorAttribute => println!("Actor Attribute: {}", inner_pair.as_str()),
                    Rule::number => println!("Number: {}", inner_pair.as_str()),
                    Rule::identifier => println!("Identifier: {}", inner_pair.as_str()),
                    Rule::string => println!("String: {}", inner_pair.as_str()),
                    Rule::inner => println!("Inner String: {}", inner_pair.as_str()),
                    Rule::char => println!("Char: {}", inner_pair.as_str()),
                    _=> println!("Error!!!")
                }
            }
        }

}