# Note: `auth_controller` login syntax fix

This document records the syntax issue that previously existed in `server/src/controllers/auth_controller.rs` in the `login` function.
The current branch already reflects the fix, so this file serves as a repair note rather than a design change.

## Issue

- `login` mixed an `if verified { ... }` block with leftover `match` arms
- Branches such as `Ok(false) => ...` and `Err(e) => ...` remained after the closing brace of the `if` block
- That structure was invalid Rust syntax

## Cause

- `verify_password(...)?` yields a `bool`
- After the `?`, the result is no longer a `Result`
- Therefore the logic should be handled with `if / else`, not with `match` arms for `Ok` and `Err`

## Fixed Shape

- If verification succeeds, the session is created and `200 OK` is returned
- If verification fails, `401 Unauthorized` is returned
- No `Ok(false)` / `Err(e)` branches remain in the `login` body

## Status

- This file is a historical note, not a proposal for a new behavior
