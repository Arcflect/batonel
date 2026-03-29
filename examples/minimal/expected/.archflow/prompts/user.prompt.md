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
- The entity strictly protects its domain invariants.
- Methods represent business rules, not just generic getters/setters.
- No application, transport, or persistence details leak into this layer.

