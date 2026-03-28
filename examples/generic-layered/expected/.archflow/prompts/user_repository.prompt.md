# Artifact Prompt: user_repository

Implement the `user_repository` artifact.

## Role
repository_interface

## Module
user

## Responsibilities
- Define an abstraction for persistence access
- Provide a repository boundary for user persistence
- Hide infrastructure details from the application layer

## Must not
- Contain persistence implementation details
- Contain transport-specific logic

## Allowed dependencies
- src/domain
- src/application

## Forbidden dependencies
- src/interfaces
- src/infrastructure

## Outputs
- UserRepository

## Completion criteria
- The artifact defines a persistence abstraction clearly
- It contains no infrastructure implementation
- The boundary is usable by application services