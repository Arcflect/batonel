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
- Depend on transport or persistence details
- Contain HTTP-specific logic
- Contain direct database access

## Allowed dependencies
- domain

## Forbidden dependencies
- interfaces
- infrastructure
- application

## Outputs
- User

## Completion criteria
- The artifact models a domain entity clearly
- No transport or persistence concern leaks into the entity
- The implementation keeps domain rules local to the entity