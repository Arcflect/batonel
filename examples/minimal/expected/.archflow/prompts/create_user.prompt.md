# Artifact Prompt: create_user

Implement the `create_user` artifact.

## Role
usecase

## Module
user

## Responsibilities
- Execute one application use case
- Coordinate domain behavior
- Accept a user creation request
- Persist through an abstract repository boundary

## Must not
- Access infrastructure details directly
- Return transport-specific responses
- Write SQL directly

## Allowed dependencies
- domain
- application

## Forbidden dependencies
- interfaces
- infrastructure

## Inputs
- CreateUserCommand

## Outputs
- CreateUserResult

## Completion criteria
- The usecase implements exactly one application flow.
- It coordinates domain behavior through ports but does not implement infrastructure natively.
- No HTTP or database logic is present.

