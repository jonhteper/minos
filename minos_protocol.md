***Warning ⚠️ ️: this text is a draft***

*Original in Spanish*

# The minos protocol

This is an authorization protocol. It is designed with object-oriented programming in mind.

## Definitions

The protocol is based on four fundamental concepts.

* **Resource**: any manipulable entity in a system.
* **Agent**: the one that manipulates the entities.
* **Group**: groupings of `agents`, agents can belong to more than one `group` or not belong to any of them.
* **Authorization**: certificate issued to an `agent` indicating how it can manipulate a given `resource`.

### Resource

Here enter the business logic entities that need to go through an authorization process to be manipulated. Each `resource` must have the following attributes:

* **Unique identifier**: in order to differentiate between each resource;
* **Owner** (optional): identifier, name or signature of the owner;
* **Authorization policies/rules**: each `resource` defines how `agents` may manipulate it;
* **Resource type** (optional): indicates what type of entity it is, proposed as an additional way to identify resources, as well as to facilitate the loading of authorization rules for entities of the same type.

### Agent

Entity that is able to manipulate the `resources`. In a typical system it is usually a user that manipulates resources, however this definition allows any entity to be an `agent`, including another `resource`.

Each `agent` must have the following attributes:

* **Unique identifier**;
* **List of groups** to which it belongs.

### Group

The protocol does not specify the behavior of groups, they are only expected to contain one or more `agents` since this division by groups is used as an authorization method. Groups only need:

* **Unique identifier**.

### Authorization

Authorizations in the protocol are intended to be both transmitted and stored and need to be bound to a single `resource` and a single `agent`.

Authorizations require the following attributes:

* **permissions**: they indicate the actions that the `agent` can perform with the `resource`, they are not limited to the CRUD set, but can represent any action.
* **identifier of the `agent`**.
* **resource identifier**.
* **resource type** (optional).
* **Expiration**: to facilitate the transmission of authorizations between different computer systems and to facilitate the administration and revocation of the authorization, it is obligatory to establish a deadline on which the authorization is valid.

### Authorization

Actions that `agents` can perform on `resources`.

## Authorization policies

Minos proposes three types of authorization, by means of the `resource` owner, by means of membership of a specific `group` and on demand. Resources can have an unlimited number of authorization policies, so it is recommended to be specific with the permissions granted to the `agent` and the duration of those permissions.

### By owner

Intended for all those `resources` that effectively have an owner, it is intended to verify that the `agent` is the owner of the `resource`.

### By group

A white list with the identifiers of the groups to which the agent can belong to obtain the authorization.

### Custom

Any algorithm other than or complementary to the previous two.

## Security

To ensure the security of this protocol, a system must be in place that is aware of the `resources` and `agents`; that is capable of transmitting authorizations and validating the authorizations presented to it. It is recommended that it should preferably run in an isolated environment and that other programs should only be able to interact with it through an API. For the transmission of `authorizations` it is recommended to use the JWT standard or, failing that, to store the `authorizations` in the authorization server.
