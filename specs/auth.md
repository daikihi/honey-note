# User and Authentication Specification

This document describes the user model, authentication flow, and session structure used by the current branch of honey-note.

## User Model

- A user is the authenticated actor of the system
- `username` is the login identifier and is normalized to lowercase
- `email` is not stored in plain text
- `password` is stored as a bcrypt hash
- `display_name` is shown in the UI
- A user with `terminated_at` set is treated as inactive

## Database

### `users`

- `id`
- `username`
- `email_hash`
- `password_hash`
- `display_name`
- `created_at`
- `terminated_at`
- `updated_at`

### Constraints

- `username` is unique
- `email_hash` is unique
- `updated_at` is maintained by an update trigger

### User-owned tables

- `honey`
- `beekeeper`
- `flower`

Each of these tables has a `user_id` column so records can be separated per user.

## Session

- Login state is stored in the `honey_note_session` cookie
- Session data uses `SessionData`
- `SessionData` contains
  - `version`
  - `user_id`
  - `username`

## Authentication Flow

### Sign up

- `POST /api/auth/signup`
- Accepts `username`, `email`, `password`, and `display_name`
- Fails on validation errors or duplicates

### Log in

- `POST /api/auth/login`
- Accepts `username` and `password`
- Creates a session on success

### Log out

- `POST /api/auth/logout`
- Clears the session

### Current user

- `GET /api/auth/me`
- Returns the current login state and the current user's identifiers

## Authorization

- Protected endpoints use the `AuthenticatedUser` extractor
- Missing or invalid sessions return `401 Unauthorized`
- Detail and update operations check ownership before proceeding
