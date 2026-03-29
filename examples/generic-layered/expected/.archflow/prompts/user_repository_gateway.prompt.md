# Artifact Prompt: user_repository_gateway

Implement the `user_repository_gateway` artifact.

## Role
gateway

## Module
user

## Responsibilities
- Implement infrastructure-facing persistence access
- Translate between persistence structures and domain/application models
- Support user persistence operations

## Must not
- Contain transport-specific logic
- Define core business policy
- Act as an application service

## Allowed dependencies
- src/domain
- src/application
- src/infrastructure

## Forbidden dependencies
- src/interfaces

## Outputs
- UserRepositoryGateway

## Completion criteria
- The artifact focuses exclusively on its defined responsibilities.
- The implementation respects forbidden dependencies and architectural rules.

