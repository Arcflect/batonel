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
- The artifact defines an outbound port clearly
- It contains abstraction only, not infrastructure details
- The boundary is usable by application usecases