# User and Authentication Specification

This document describes the user model, authentication flow, and session structure used by the current branch of honey-note.

## User Model

| Item | Description |
| :--- | :--- |
| Actor | The authenticated user of the system |
| `username` | Login identifier, normalized to lowercase |
| `email` | Not stored in plain text |
| `password` | Stored as a bcrypt hash |
| `display_name` | Displayed in the UI |
| `terminated_at` | Marks an inactive user |

## Database

### `users`

| Column | Description |
| :--- | :--- |
| `id` | Primary key |
| `username` | Unique login identifier |
| `email_hash` | Unique hash used for duplicate checks |
| `password_hash` | Bcrypt hash of the password |
| `display_name` | UI display name |
| `created_at` | Creation timestamp |
| `terminated_at` | Inactive timestamp |
| `updated_at` | Auto-updated timestamp |

### Constraints

| Constraint | Description |
| :--- | :--- |
| `username` UNIQUE | Usernames must be unique |
| `email_hash` UNIQUE | Email hashes must be unique |
| `updated_at` trigger | Updated automatically on row updates |

### User-owned tables

| Table | Meaning |
| :--- | :--- |
| `honey` | Honey records owned by a user |
| `beekeeper` | Beekeeper records owned by a user |
| `flower` | Flower records owned by a user |

Each of these tables has a `user_id` column so records can be separated per user.

## Session

| Item | Description |
| :--- | :--- |
| Cookie name | `honey_note_session` |
| Stored object | `SessionData` |

### `SessionData`

| Field | Description |
| :--- | :--- |
| `version` | Version number for future compatibility |
| `user_id` | Logged-in user ID |
| `username` | Logged-in username |

## Authentication Flow

| Step | Endpoint | Summary |
| :--- | :--- | :--- |
| Sign up | `POST /api/auth/signup` | Accepts `username`, `email`, `password`, and `display_name`. Fails on validation errors or duplicates |
| Log in | `POST /api/auth/login` | Accepts `username` and `password`. Creates a session on success |
| Log out | `POST /api/auth/logout` | Clears the session |
| Current user | `GET /api/auth/me` | Returns the current login state and the current user's identifiers |

## Authorization

| Rule | Description |
| :--- | :--- |
| Protected endpoints | Use the `AuthenticatedUser` extractor |
| Missing or invalid session | Returns `401 Unauthorized` |
| Ownership checks | Detail and update operations check ownership before proceeding |
