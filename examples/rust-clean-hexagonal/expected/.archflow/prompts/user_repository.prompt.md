# Artifact Prompt: user_repository

Implement the `user_repository` artifact.

## Role
repository_port

## Module
user

## Responsibilities
- Define an outbound persistence boundary
- Provide an abstraction for user persistence
- Hide infrastructure-specific details behind an abstraction

## Must not
- Contain database-specific implementation
- Contain HTTP-specific logic

## Allowed dependencies
- crates/domain
- crates/application

## Forbidden dependencies
- crates/adapters/http
- crates/adapters/db

## Outputs
- UserRepository

## Completion criteria
- The abstraction focuses purely on the repository intent (e.g., retrieving aggregates).
- It is fully decoupled from specific SQL, ORM, or database terminology.

