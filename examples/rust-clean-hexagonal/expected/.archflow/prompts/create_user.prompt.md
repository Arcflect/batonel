# Artifact Prompt: create_user

Implement the `create_user` artifact.

## Role
usecase

## Module
user

## Responsibilities
- Execute one application use case
- Coordinate domain behavior through ports
- Accept a user creation command
- Persist through the user repository port

## Must not
- Access adapter implementations directly
- Contain HTTP request/response logic
- Write SQL directly

## Allowed dependencies
- crates/domain
- crates/application/src/ports

## Forbidden dependencies
- crates/adapters/http
- crates/adapters/db

## Inputs
- CreateUserCommand

## Outputs
- CreateUserResult

## Completion criteria
- The usecase implements exactly one application flow.
- It coordinates domain behavior through ports but does not implement infrastructure natively.
- No HTTP or database logic is present.

