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
- Depend on application or adapter concerns
- Contain HTTP or database-specific logic
- Contain persistence access

## Allowed dependencies
- crates/domain

## Forbidden dependencies
- crates/application
- crates/adapters/http
- crates/adapters/db

## Outputs
- User

## Completion criteria
- The artifact clearly models a domain entity
- Domain invariants are kept local to the entity
- No application or adapter concern leaks into this artifact