# Code-style rules for this repository

This file is a specification, not compiled code: it exists so that every
Rust rule has a single, greppable home under `docs/`. `AGENTS.md` points
here; both human contributors and agents must consult it before writing
or reviewing Rust code in any crate of this workspace.

Where the standard `rustfmt` output or the workspace clippy configuration
already encodes a rule mechanically, this file records the *intent* and
the review checklist; the tooling remains the arbiter.

## Imports

### 1. Import through the crate root; use `super::*` in unit tests.

Do not write `use super::...` in production code. Relative-to-parent
imports couple a file to its physical
location: moving a module (or promoting a file out of a directory)
silently changes what the import resolves to, and `super::super::`
chains are unreadable.

Instead, import through the crate root or an explicitly named module:

Bad:

```rust
use super::{MCType, ProtocolError};
use super::super::Nbt;
use super::*; // forbidden in production code
```

Good:

```rust
use crate::{MCType, ProtocolError};
use crate::nbt::Nbt;
use crate::packets::status::serverbound::StatusRequest;
```

Exception: inside unit test modules (`#[cfg(test)] mod tests`), use
`use super::*;` to access the code under test and its imports. Prefer this
over repeating long `crate::...` paths or lists of parent-module items:
the tests intentionally belong to the module they exercise. Import any
additional test dependencies explicitly. This exception does not permit
`super::` imports in production code or chains such as `super::super::`.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // Tests exercise the parent module's items directly.
}
```

### 2. Single import point at the crate root.

Consumers import from `mclib::{...}` (the crate root re-exports every
public type). Deeper paths such as `mclib::types::var_int::MCVarInt`
are reserved for module-internal use inside `mclib` itself.

## Casts and conversions

### 3. No lossy `as` casts — except bit-pattern reinterpretation.

The workspace clippy configuration denies the lossy-cast lints
(`cast_possible_truncation`, `cast_sign_loss`, `cast_possible_wrap`,
`cast_lossless`). Conversions must be explicit:

- Lossless widening: `From`/`Into`.
- Potentially lossy narrowing or sign changes: `try_from` with a typed
  `ProtocolError`.
- Extracting seven bits for VarInt/VarLong: `(value & 0x7f) as u8`
  needs no allowance: Clippy recognizes that the mask proves the result
  fits. Do not invent an error path for this operation.
- Single-byte extraction from signed integers: `to_le_bytes()[0]` /
  `from_le_bytes` — no cast needed, the intent reads directly.
- Same-width integer reinterpretation (`i32` <-> `u32`, `i64` <-> `u64`)
  for two's-complement wire formats (VarInt/VarLong): prefer
  `cast_unsigned()` / `cast_signed()`, which preserve the bit pattern
  without lint allowances. Avoid `from_ne_bytes`/`to_ne_bytes` round-trips.
- Sign extraction from packed words (e.g. `(packed >> 38) as i32`) is a
  legitimate `as` use: it is a documented two's-complement shift, not a
  value conversion.
- `i32::MAX as usize` in `const` contexts (default generic parameters,
  const max bounds) is also legitimate: `TryFrom` is not a `const fn`,
  so `try_from` is unavailable. No inline justification is needed;
  never use this escape hatch in runtime code.

### 4. Protocol constants are written literally.

VarInt bit masks appear as `0x7f` and `0x80` directly in the code, not
behind named constants. Domain constants that have protocol meaning
(max packet size, dimension bounds) do get names.

## Errors

### 5. No `unwrap()`/`expect()` in production paths.

Workspace clippy denies `unwrap_used` and `expect_used`. Handle I/O,
input, and operational failures explicitly with typed errors
(`ProtocolError` in `mclib`). Tests may use `unwrap` freely: clippy is
run on production code only (`cargo clippy --workspace`, without
`--all-targets`), and test files must not carry clippy allow
attributes.

## Intent

### 6. Code expresses exact intent; it never outsmarts the linter.

A lint exists to force a decision into the open. The response to a lint
is either to write the intent more clearly or to justify the exception
— never to dodge the check:

- Do not scatter `#[allow]` attributes across call sites just to make
  a warning go away. A suppression is justified only when it lives in
  the operation that needs it, with a comment explaining the invariant.
  Direct bit-pattern casts in VarInt/VarLong codecs need no wrapper
  functions. Do not add helpers solely to contain lint attributes.
- Do not use fragile workarounds whose only purpose is to slip past a
  lint heuristic — `from_ne_bytes(to_ne_bytes())` round-trips, dummy
  borrows, double negations, magic `.cast()` chains. If the reason a
  line exists is "so clippy stays quiet", the line is wrong; name the
  operation instead.
- When a rule in this file sounds like a heuristic to satisfy rather
  than a property of the code, prefer restructuring until the property
  holds naturally.

Tests are the exception to suppression placement only in the sense
that clippy does not run on them; the same intent rule applies.

## Module layout

### 7. Keep `mod.rs` files declarative.

A `mod.rs` contains module declarations, re-exports, and module-level
documentation — no traits, no types, no logic. Cross-cutting contracts
get their own file (see `codec.rs` for `MCType`, `error.rs` for the
error types, both at the `mclib` crate root).

### 8. Errors live in `error.rs`.

Each crate keeps its error enums in `src/error.rs` at the crate root
and re-exports them from `lib.rs`.
