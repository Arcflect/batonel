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
- The entity strictly protects its domain invariants.
- Methods represent business rules, not just generic getters/setters.
- No application, transport, or persistence details leak into this layer.

