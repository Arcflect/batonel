# Artifact Prompt: create_user_handler

Implement the `create_user_handler` artifact.

## Role
http_handler

## Module
user

## Responsibilities
- Translate HTTP input into application input
- Call the create_user usecase
- Translate application output into HTTP response data

## Must not
- Contain direct persistence logic
- Embed business rules that belong in domain or usecase
- Write SQL directly

## Allowed dependencies
- crates/application
- crates/adapters/http

## Forbidden dependencies
- crates/adapters/db

## Inputs
- CreateUserHttpRequest

## Outputs
- CreateUserHttpResponse

## Completion criteria
- The handler cleanly translates transport models into application requests.
- It invokes the application layer but embeds zero core business rules locally.

