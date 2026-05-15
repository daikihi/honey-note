# API Specification

This document describes the API specification of the current branch of honey-note.
It focuses on the behavior that is already implemented, including endpoints, authentication requirements, and the main request/response shapes.

## Common Rules

- The base API prefix is `/honey-note/api`
- Authenticated endpoints read the current user from the session
- Unauthenticated access to protected endpoints results in `401 Unauthorized`
- `PUT` requests are accepted as JSON and their bodies are logged
- Responses are primarily JSON

## Authentication

### `POST /api/auth/signup`

- Registers a new user
- Request fields
  - `username`
  - `email`
  - `password`
  - `display_name`
- Behavior
  - `username` is normalized to lowercase
  - `email` is hashed with SHA-256 for duplicate checks
  - `password` is hashed with bcrypt
  - `display_name` falls back to `username` if omitted
- Typical responses
  - `200 OK`
  - `400 Bad Request`
  - `500 Internal Server Error`

### `POST /api/auth/login`

- Logs in a user and creates a session
- Request fields
  - `username`
  - `password`
- Behavior
  - `username` is normalized to lowercase before lookup
  - On success, `SessionData` is stored under the `user` key
- Typical responses
  - `200 OK`
  - `401 Unauthorized`

### `POST /api/auth/logout`

- Clears the session
- Typical response
  - `200 OK`

### `GET /api/auth/me`

- Returns the current login state
- Typical response
  - `200 OK`

## Master Data

### `GET /honey-note/api/prefectures`

- Returns all prefectures
- Authentication is not required

## Beekeepers

### `GET /honey-note/api/beekeepers`

- Returns the authenticated user's beekeepers

### `GET /honey-note/api/beekeeper/{id}`

- Returns one beekeeper by ID
- Returns not found when the current user does not own the record

### `PUT /honey-note/api/beekeeper/new`

- Creates a beekeeper
- Request body
  - `Beekeeper`

### `PUT /honey-note/api/beekeeper/edit/{id}`

- Updates a beekeeper by ID
- Request body
  - `Beekeeper`

## Flowers

### `GET /honey-note/api/flowers`

- Returns the authenticated user's flowers

### `GET /honey-note/api/flower/{id}`

- Returns one flower by ID
- Returns not found when the current user does not own the record

### `PUT /honey-note/api/flower/new`

- Creates a flower
- Request body
  - `Flower`

### `PUT /honey-note/api/flower/edit/{id}`

- Updates a flower by ID
- Request body
  - `Flower`

## Honeys

### `GET /honey-note/api/honeys`

- Returns the authenticated user's honeys

### `GET /honey-note/api/honey/{id}`

- Returns one honey by ID
- Returns not found when the current user does not own the record

### `PUT /honey-note/api/honey/new`

- Creates a honey record
- Request body
  - `HoneyNewRequest`
  - `basic`
  - `dynamic`
  - `created_at`
- Behavior
  - Automatically creates and links a beekeeper if needed
  - Automatically creates and links flowers if needed

### `PUT /honey-note/api/honey/edit`

- Updates a honey record
- Request body
  - `HoneyEditRequest`
  - `id`
  - `basic`
  - `dynamic`
  - `updated_at`

## Other

### `GET /health`

- Returns a server health check response
