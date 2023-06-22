***Warning ⚠️ ️: This project is active development***

# minos

Authorization library for Minos authorization language. This repository contains the languaje definition
and the interpreter engine.

## Minos Language
The Minos lang is a Domain-specific language, created for write authorization policies.

For example:
```minos
sintaxis=0.15;

/* Multi-line

    Commentary

    Example

*/
env ProductUseCases {
    resource Product {
        policy {
            allow = ["create", "delete"]; /* One-line commentary example */

            rule {
                actor.type = RootUser;
            }

            rule {
                actor.groups *= ["sales"];
                actor.roles = ["admin"];
            }
        }

        policy {
            allow = ["view", "add to cart"];

            rule {
                actor.type = CostumerUser;
            }

        }
    }
}
```


