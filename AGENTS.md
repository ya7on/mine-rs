# Repository Instructions

## Living documentation

- Treat `AGENTS.md` as a concise instruction and navigation layer. Keep detailed design documentation under `docs/`.
- Repository-wide documentation belongs in `docs/`. Document each significant workspace crate under `docs/crates/<crate-name>/` when doing so provides useful context.
- Before modifying a crate, read its relevant documentation under `docs/crates/<crate-name>/`. For changes that cross crate boundaries, also read the repository-level architecture documentation.
- Keep documentation synchronized with code. When a change affects documented architecture, responsibilities, dependencies, runtime behavior, lifecycle, public interfaces, external interactions, or invariants, update the relevant documentation in the same change.
- When introducing a new crate or significant architectural concept, create the relevant documentation if future work would benefit from preserving that context.
- Document semantic structure, constraints, rationale, and important relationships. Do not duplicate source layouts or implementation details that are readily discoverable from the code.
- Create only documentation that carries useful information. A crate does not need every document listed below.

Crate documentation may include:

- `README.md` — purpose, responsibilities, important modules, and links to deeper documentation.
- `architecture.md` — boundaries, dependencies, invariants, internal architecture, and significant design decisions.
- `runtime.md` — lifecycle, background tasks, concurrency, I/O, runtime behavior, and external interactions.

## Shell commands

- Prefer simple, explicit commands that are easy to review, approve, and reuse.
- Run independent operations as separate commands. Avoid chaining with `&&`, `;`, command substitutions, or complex pipelines when separate calls are sufficient.
- Avoid large one-liners and temporary shell scripts for straightforward tasks.
- Prefer stable command forms such as `cargo test`, `cargo check`, and `cargo clippy` that can be safely allowlisted and repeated.
- Use compound shell commands only when they provide a clear technical benefit; keep their scope narrow and explain non-obvious behavior.

## Scope

- Implement only what the current task requires. Do not add speculative features, extension points, or abstractions.
- Prefer the smallest complete change that respects the existing architecture and leaves the repository in a working state.
- Document significant architectural decisions as part of the change that introduces them.

## Git

- Write commit messages in Conventional Commits format with a type prefix, for example `feat: add packet framing`.
- Keep commit subjects short and focused on the change.
- Do not push commits unless explicitly requested.

## Rust

- `docs/codestyle.md` is the authoritative Rust style specification. Consult it
  before writing or reviewing Rust code in this repository, and check new code
  against it: no `super::` imports in production; prefer `super::*` in unit
  tests; no lossy `as` casts, literal
  protocol bit masks, declarative `mod.rs` files, and errors in each crate's
  `error.rs`. Details and rationale live in that file.
- Avoid `unsafe` unless it is necessary, narrowly scoped, and explicitly justified.
- Do not use `unwrap()` or `expect()` in production paths unless failure would prove an internal invariant violation. Handle input, I/O, and operational failures explicitly.
- Use domain types instead of primitive values when protocol concepts have distinct semantics.
- Keep crate boundaries meaningful. Do not introduce cross-crate dependencies solely for implementation convenience.

## Protocol

- Treat the Minecraft protocol specification as an external contract. Verify packet IDs, layouts, field types, state transitions, and version-specific behavior instead of guessing.
- Keep protocol encoding and decoding separate from networking, server state, and game logic.
- Treat client input as untrusted. Validate frame and collection bounds before allocation or use of decoded values.
- Prefer tests with known protocol byte sequences for codecs and packet framing where practical.
- Document non-obvious protocol behavior and compatibility assumptions in the relevant crate documentation.

## Architecture

- Prefer clear ownership and message/data flow over shared mutable state.
- Do not bypass established crate boundaries with convenience dependencies or duplicated internal knowledge.
- Introduce an abstraction only when it solves a present, demonstrated problem.

## Verification

Run applicable checks as separate commands after making changes:

- `cargo fmt --check`
- `cargo check --workspace`
- `cargo clippy --workspace` (lib and binaries only; tests are covered by `cargo test`)
- relevant tests for the changed crate or behavior

Clippy is run without `--all-targets` and without `--tests`: the strict lint
level (pedantic, nursery, `unwrap_used = deny`) applies to production code,
while tests are verified by `cargo test` alone. Test code must not carry
clippy allow attributes.

Do not fix unrelated warnings or failures unless they prevent verification of the requested change. Report such blockers instead.
