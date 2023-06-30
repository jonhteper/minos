***Warning ⚠️ ️: This project is active development***

# minos

Authorization library for Minos authorization language. This repository contains the language definition, the parser and the authorization engine.

## Minos authorization

...

## Minos Language
The Minos lang is a Domain-specific language, created for write authorization policies.

To describe the syntax we start with the next file:
```minos
/* in example.minos */
syntax = 0.16;

/* Resource declaration */
resource User {
	/* Environment declaration */
    env DEFAULT {
    	/* policy declaration */ 
        policy {
        	/* Permissions granted if at least one rule is met */
            allow = ["create", "read", "update", "delete"];
			
			/* Rule declaration */
            rule {
                actor.type = RootUser;
            }

            rule {
                actor.type = resource.type;
                actor.id = resource.id;
            }
        }
    }
}

```

First, comments in the minos files are written between the `/*` and `*/` characters, as in CSS.

```minos
/*in example.minos */
```



Each minos file needs to declare the version of the syntax used at the beginning of the document.

```minos
syntax = 0.16;
```

...

Now, we can comeback to the all file:

```minos
syntax = 0.16;

resource User {
    env DEFAULT {
        policy {
            allow = ["create", "read", "update", "delete"];
            rule {
                actor.type = RootUser;
            }
            rule {
                actor.type = resource.type;
                actor.id = resource.id;
            }
        }
    }
}

```

The block describes a resource named `User` with an only one environment named `DEFAULT`. Is important to indicate that if the `DEFAULT` block exist, will be apply in every authorization process.

### Implicit environment

Based on the above, we can rewrite the block without environments:

```minos
resource User {
    policy {
        allow = ["create", "read", "update", "delete"];
        rule {
            actor.type = RootUser;
        }
        rule {
            actor.type = resource.type;
            actor.id = resource.id;
        }
    }
}
```

The code will be parsed equals to previous example but in this case the `DEFAULT` block is implicit. This feature is named "implicit default".

### Named environments

The environments provide encapsulation for policies. To exclude an "default behavior" pattern, is possible use named environments. We can modify the example, and define two environments: `Testing` and `Production`.

```minos
resource User {
    env Testing {
        policy {
            allow = ["create", "read", "update", "delete"];
            rule {
                actor.type = RootUser;
            }
            rule {
                actor.type = resource.type;
                actor.id = resource.id;
            }
        }
    }
    env Production {
        policy {
            allow = ["create", "read", "update", "delete"];
            rule {
                actor.type = resource.type;
                actor.id = resource.id;
            }
        }
    }
}
```

We can see, that in the `Testing` environment, a `RootUser` can manipulate the `Users`, but in 
`Production` only the `User` can manipulate it self.

Is important to mention that when exist two or more environment, use the implicit `DEFAULT` isn't possible. Since in this case the `DEFAULT` block not exists and is necessary to indicate the environment's name in the authorization process.

Now, if we are sure that only need two environments, we can rewrite the code, using the `DEFAULT` environment.

```minos
resource User {
    env DEFAULT {
        policy {
            allow = ["create", "read", "update", "delete"];
            rule {
                actor.type = resource.type;
                actor.id = resource.id;
            }
        }
    }
    env Testing {
        policy {
            allow = ["create", "read", "update", "delete"];

            rule {
                actor.type = RootUser;
            }
        }
    }
}
```

This pattern makes it clear that it is an edge case, and allow us to remove the `Testing` environment before deployment. But, we only do it because the rules are compatible and we **want** to use the `DEFAULT` environment.

### Specification

What if, we need a different behavior based not on environments but on the resources themselves? This is possible using the *specification* feature and the *attributed resources*.

In the following example, we can two different blocks:

```minos
resource File {
    policy {
        allow = ["read"];
        rule {
            actor.type = User;
        }
    }
    policy {
        allow = ["write", "delete"];

        rule {
            actor.roles *= ["admin"];
        }
    }
}

resource File {
    id = "confidential.john.data.file.id";
    policy {
        allow = ["read"];

        rule {
            actor.id = "john.user.Id";
        }
    }
}

```

The first block defines a resource named `File`. Based on  the rules, the `Users` can read a `File` but only an actor with role "admin" can write or delete a `File`.

In the second block, we find another declaration for `File`. We can see some diferencies:
* the resource have an attribute named `id`, and for this is an *attributed resource*;
* the resource have a policy that contains a *conflictive* rule, because the permission "read" is granted with two policies in the same environment.

How the authorization process works in this case? If the resource is a `File` and its id is "confidential.john.data.file.id", the  rules for `File` will be ignored, and will only apply the policy with the matching id. The consequent of this, is that the `File` with id "confidential.john.data.file.id" can't be overwrited nor deleted.

It is simple, if we want to define special rules for an specific resource, we just use the *specification*.

### Use of id attribute in rules

But If we want use the rules into `DEFAULT` environment instead of ignore it, we can extend the first blocks like this:

```minos
resource File {
    policy {
        allow = ["read"];
        rule {
            actor.type = User;
        }
        /* start of adition --- */
        rule {
        	resource.id = "confidential.john.data.file.id";
        	actor.id = "john.user.Id";
        }
        /* --- end of adition */
    }
    policy {
        allow = ["write", "delete"];

        rule {
            actor.roles *= ["admin"];
        }
    }
}
```

The above code will be parsed since *the actor with id "john.user.Id" can read the `File` with id "confidential.john.data.file.id"*. We can see that the if the actor is an `User`, the new rule is really useless. This is a problem with the use of `actor.id` and/or `resource.id` in the same environment: the restrictive rules can be skipped for more relaxed rules.

### Attributes comparison

The use of id attribute in the last example can be useful only in case of the actor isn't an `User`. So, we can work with the next data, using the above code:

```json
{
    "actor": {
    	"id": "john.user.Id",
        "type": "Employee",
        "groups": ["employees"],
        "roles": ["admin"]
    },
    "resource": {
        "id": "confidential.john.data.file.id",
        "type": "File",
        "owner": "john.user.Id"
    },
    "permissions": ["read"]
}
```

The authorization is granted, because the rules cover this case exactly. So, what if we have one confidential file for every employee? We must be to write one rule for every file-employee couple? Not really, we can use the `actor.owner` attribute, that's be convenient supported in currently minos lang version (v0.16).

```minos
resource File {
    policy {
        allow = ["read"];
        rule {
            actor.type = User;
        }
        /* start of change --- */
        rule {
        	resource.owner = actor.id;
        }
        /* --- end of change */
    }
    policy {
        allow = ["write", "delete"];

        rule {
            actor.roles *= ["admin"];
        }
    }
}
```

Additionally, is possible to enhance the rule, adding the `actor.type` requirement:

```minos
/*... */
rule {
    actor.type = Employee;
	resource.owner = actor.id;
}
/*...*/
```

The attribute comparison is an excellent way to avoid the common places logic duplication. Unfortunately, in currently minos lang version (v0.16) isn't many supported attributes. But we hope to improve support for this feature in future releases.

### Parsing rules

At this point it is important to explain how minos parser works with blocks with the same identifier. For example, the code of [this section](#Use of id attribute in rules) can be rewrite like this:

```minos
resource File {
    policy {
        allow = ["read"];
        rule {
            actor.type = User;
        }
        rule {
        	resource.id = "confidential.john.data.file.id";
        	actor.id = "john.user.Id";
        }
    }
}

resource File {
    policy {
        allow = ["write", "delete"];

        rule {
            actor.roles *= ["admin"];
        }
    }
}
```

How minos parses the above text?

1. Search for resources.
2. Add `File` resource.
3. Search for environments.
4. Add implicit `DEFAULT` environment.
5. Search for policies.
6. Add policy to allow permissions `["read"]` and two rules.
7. Search for more policies.
8. Skip searching for more environments, because the block uses  implicit `DEFAULT`.
9. Search for more resources.
10. Find `File` resource.
11. Search for environments.
12. Find implicit DEFAULT environment.
13. Search for policies.
14. Add policy to allow permissions `["write", "delete"]` and one rule.
15. Search for more policies.
16. Skip searching for more environments, because the block uses  implicit `DEFAULT`.
17. Search for more resources ...

This behavior is the same with resources in other files. For example, we can rewrite the above code like this:

```minos
/* in file.minos */
resource File {
    policy {
        allow = ["read"];
        rule {
            actor.type = User;
        }
    }
}
```

```minos
/* in file2.minos */
resource File {
    policy {
        allow = ["read"];        
        rule {
        	resource.id = "confidential.john.data.file.id";
        	actor.id = "john.user.Id";
        }
    }
}
```

```minos
/* in file3.minos */
resource File {
    policy {
        allow = ["write", "delete"];

        rule {
            actor.roles *= ["admin"];
        }
    }
}
```

The final parsing, will be exactly the same that the first example in this section.





