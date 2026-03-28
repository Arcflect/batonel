# Artifact Prompt: create_user_service

Implement the `create_user_service` artifact.

## Role
service

## Module
user

## Responsibilities
- Execute one application-level operation
- Coordinate domain behavior through abstractions
- Accept a user creation input
- Persist through the repository interface

## Must not
- Contain transport-specific behavior
- Access infrastructure implementations directly
- Embed persistence implementation logic

## Allowed dependencies
- src/domain
- src/application/interfaces

## Forbidden dependencies
- src/interfaces
- src/infrastructure

## Inputs
- CreateUserInput

## Outputs
- CreateUserOutput

## Completion criteria
- The artifact has one clear application responsibility
- It uses persistence only through the repository interface
- No interface or infrastructure implementation leaks into this artifact