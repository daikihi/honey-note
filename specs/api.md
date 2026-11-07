# API Specification

This document summarizes the API specification of the current branch of honey-note.
It is organized as a table-first reference so that endpoints, authentication requirements, and main inputs/outputs are easy to scan.

## Common Rules

| Item | Value |
| :--- | :--- |
| Base URL | `/api/auth` for authentication; `/honey-note/api` for other data APIs |
| Authentication model | Read the current user from the session |
| Failure when unauthenticated | `401 Unauthorized` |
| PUT requests | Accepted as JSON and logged with their bodies |
| Main response format | JSON |

## Authentication

| Method | Path | Auth | Main Input | Main Behavior | Typical Responses |
| :--- | :--- | :--- | :--- | :--- | :--- |
| POST | `/api/auth/signup` | No | `username`, `email`, `password`, `display_name` | Normalize `username` to lowercase. Hash `email` for duplicate checks. Hash `password` with bcrypt. Use `username` as `display_name` if omitted | `200 OK`, `400 Bad Request`, `500 Internal Server Error` |
| POST | `/api/auth/login` | No | `username`, `password` | Normalize `username` to lowercase and verify credentials. On success, store `SessionData` under the `user` key | `200 OK`, `401 Unauthorized` |
| POST | `/api/auth/logout` | No | None | Clear the session | `200 OK` |
| GET | `/api/auth/me` | No | None | Return login state and, when logged in, `user_id`, `username`, and `display_name`; logged-out values are `null` | `200 OK`, `401 Unauthorized` (session user no longer exists), `500 Internal Server Error` |
| PUT | `/api/auth/me/display-name` | Yes | JSON: `display_name` (string) only | Trim and update the session user's display name. Reject blank values and extra fields. Require a same-origin `Origin` header | `200 OK` (`display_name`), `400 Bad Request`, `401 Unauthorized`, `403 Forbidden`, `404 Not Found`, `500 Internal Server Error` |

Successful sign-up and display-name saves write structured recovery log events with a timestamp, user ID, and stored display name. The sign-up event also includes the username.

The display-name update's same-origin check uses HTTP as the public scheme in `local` and HTTPS in `prod`. It does not use the scheme from forwarding headers.

## Master Data

| Method | Path | Auth | Main Input | Main Behavior | Typical Responses |
| :--- | :--- | :--- | :--- | :--- | :--- |
| GET | `/honey-note/api/prefectures` | No | None | Return the list of prefectures | `200 OK`, `500 Internal Server Error` |

## Beekeepers

| Method | Path | Auth | Main Input | Main Behavior | Typical Responses |
| :--- | :--- | :--- | :--- | :--- | :--- |
| GET | `/honey-note/api/beekeepers` | Yes | None | Return the authenticated user’s beekeepers | `200 OK`, `500 Internal Server Error` |
| GET | `/honey-note/api/beekeeper/{id}` | Yes | `id` | Return one beekeeper by ID. If the current user does not own the record, it is treated as not found | `200 OK`, `404 Not Found` |
| PUT | `/honey-note/api/beekeeper/new` | Yes | `Beekeeper` | Create a beekeeper | `200 OK`, `400 Bad Request` |
| PUT | `/honey-note/api/beekeeper/edit/{id}` | Yes | `id`, `Beekeeper` | Update a beekeeper by ID | `200 OK`, `400 Bad Request` |

## Flowers

| Method | Path | Auth | Main Input | Main Behavior | Typical Responses |
| :--- | :--- | :--- | :--- | :--- | :--- |
| GET | `/honey-note/api/flowers` | Yes | None | Return the authenticated user’s flowers | `200 OK`, `500 Internal Server Error` |
| GET | `/honey-note/api/flower/{id}` | Yes | `id` | Return one flower by ID. If the current user does not own the record, it is treated as not found | `200 OK`, `404 Not Found` |
| PUT | `/honey-note/api/flower/new` | Yes | `Flower` | Create a flower | `200 OK`, `400 Bad Request` |
| PUT | `/honey-note/api/flower/edit/{id}` | Yes | `id`, `Flower` | Update a flower by ID | `200 OK`, `400 Bad Request` |

## Honeys

| Method | Path | Auth | Main Input | Main Behavior | Typical Responses |
| :--- | :--- | :--- | :--- | :--- | :--- |
| GET | `/honey-note/api/honeys` | Yes | None | Return the authenticated user’s honeys | `200 OK`, `500 Internal Server Error` |
| GET | `/honey-note/api/honey/{id}` | Yes | `id` | Return one honey by ID. If the current user does not own the record, it is treated as not found | `200 OK`, `404 Not Found` |
| PUT | `/honey-note/api/honey/new` | Yes | `HoneyNewRequest` | Create a honey record. Automatically create and link a beekeeper or flowers when needed | `200 OK`, `400 Bad Request` |
| PUT | `/honey-note/api/honey/edit` | Yes | `HoneyEditRequest` | Update a honey record | `200 OK`, `400 Bad Request` |

## Other

| Method | Path | Auth | Behavior | Typical Responses |
| :--- | :--- | :--- | :--- | :--- |
| GET | `/health` | No | Return a server health check response | `200 OK` |
