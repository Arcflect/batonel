# Artifact Prompt: postgres_user_repository

Implement the `postgres_user_repository` artifact.

## Role
repository_impl

## Module
user

## Responsibilities
- Implement an outbound repository port
- Persist user data in PostgreSQL
- Translate between persistence data and domain/application models

## Must not
- Contain HTTP-specific behavior
- Define core business policy
- Act as an application usecase

## Allowed dependencies
- crates/domain
- crates/application
- crates/adapters/db

## Forbidden dependencies
- crates/adapters/http

## Outputs
- PostgresUserRepository

## Completion criteria
- The implementation fulfills an outbound port.
- It safely translates between raw persistence data and pure upstream domain models.

