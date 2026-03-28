# Artifact Prompt: user

Implement the `user` artifact.

## Role
entity

## Module
user

## Responsibilities
- Represent a core business concept
- Protect domain invariants
- Model the user entity

## Must not
- Depend on application, interface, or infrastructure concerns
- Contain transport or persistence logic
- Contain direct external access

## Allowed dependencies
- src/domain

## Forbidden dependencies
- src/application
- src/interfaces
- src/infrastructure

## Outputs
- User

## Completion criteria
- The artifact clearly models a domain entity
- Domain rules remain local to the entity
- No application, interface, or infrastructure concern leaks into this artifact