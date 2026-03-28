# Artifact Prompt: create_user_controller

Implement the `create_user_controller` artifact.

## Role
controller

## Module
user

## Responsibilities
- Accept input from an external interface
- Call the application layer
- Translate results into external response data

## Must not
- Contain business rules that belong to domain or application
- Contain direct persistence logic
- Write infrastructure-specific operations

## Allowed dependencies
- src/application
- src/interfaces

## Forbidden dependencies
- src/infrastructure

## Inputs
- CreateUserRequest

## Outputs
- CreateUserResponse

## Completion criteria
- The artifact stays thin and interface-focused
- Business rules remain outside the controller
- The controller delegates execution to the application layer