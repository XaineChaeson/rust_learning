# Agent Guidance for `rust_learning`

This file applies to the whole repository.

## Project Intent

This repository is a Rust HPC quant learning system for learners with a Python
data-science background and little or no Rust experience. Treat it as a teaching
workspace first and a production prototype second.

The single learning entrypoint is `book/README.md`. Do not create parallel
learning routes or duplicate roadmap material elsewhere unless the user asks for
a migration.

## Repository Map

- `Cargo.toml` is a virtual workspace using Rust edition 2024 and resolver 3.
- `book/` contains the curriculum, roadmap, exercises, assessments, and chapter
  source material.
- `book/chapters/XX-topic/README.md` contains the chapter text.
- `book/chapters/XX-topic/example` contains the isolated teaching crate for
  that chapter when the chapter has runnable code.
- `projects/00-bootstrap-cli` is the introductory CLI project.
- `projects/01-factor-core` is the rolling factor computation core.
- `projects/02-quant-lab-engine` is the std-only capstone engine.

## Source-of-Truth Rules

- Use `book/README.md` and `book/roadmap.md` for learning order.
- Use `book/chapter-writing-standard.md` when creating or expanding chapters.
- Use each chapter `README.md` when mapping chapters to its `example/` crate.
- Use each project `README.md` for project-specific API, test, and benchmark
  contracts.
- Keep generated or transient artifacts under ignored paths such as `target/`.
  Durable conclusions belong in tracked documentation.

## Editing Boundaries

- For chapter-specific work, modify only the matching
  `book/chapters/XX-topic/README.md` content and that chapter's `example/`
  crate unless a shared contract truly requires broader edits.
- Keep examples small and focused on the concept taught by that chapter.
- For `factor-core`, preserve borrowed input APIs, explicit error types,
  right-aligned rolling outputs, and baseline-vs-optimized correctness checks.
- For `quant-lab-engine`, preserve deterministic experiment IDs, deterministic
  ordering for parallel work, explicit boundary validation, and std-only teaching
  models unless the task explicitly moves into `book/production-residency.md`.
- Add dependencies only when the user explicitly requests an ecosystem migration
  or the existing docs already define that migration.

## Rust Code Standards

- Prefer clear baseline implementations before optimizing.
- Keep core numeric functions deterministic and side-effect free.
- Express recoverable failures with project error types instead of panics.
- Use slices and borrowing for computation inputs where the existing API does.
- Keep allocation behavior visible in APIs and documentation when performance
  matters.
- For unsafe, SIMD, FFI, threading, or scheduler logic, document the invariant
  that makes the code valid and add focused tests around the boundary.
- Preserve public API names and error semantics unless the user requested a
  breaking change.

## Documentation Standards

- Write primary learning material in Chinese, with Rust/HPC/quant terms in
  English when that matches the existing style.
- Chapters should explain concepts in this order where applicable:
  intuition, Python contrast, Rust form, code walkthrough, constraints,
  quant/HPC connection, exercises, and acceptance checks.
- Do not add vague outlines. If expanding chapter text, include runnable commands,
  expected observations, common mistakes, and validation steps.
- When documenting performance, include command, build mode, input size, window,
  repeat count, output-consistency check, speedup, and the conclusion.

## Verification

Choose the smallest verification that proves the change.

- Formatting only: `cargo fmt --check`
- One chapter crate: `cargo test -p chXX-name`
- `factor-core`: `cargo test -p factor-core`
- `quant-lab-engine`: `cargo test -p quant-lab-engine`
- Shared Rust behavior: `cargo fmt --check`, then
  `cargo clippy --workspace --all-targets -- -D warnings`, then `cargo test`

For documentation-only changes, verify links, command accuracy, and references
against the current repository structure.

## Commit Messages

When asked to commit, use the Lore protocol:

```text
<intent line: why the change was made, not what changed>

Constraint: <external constraint that shaped the decision>
Rejected: <alternative considered> | <reason for rejection>
Confidence: <low|medium|high>
Scope-risk: <narrow|moderate|broad>
Directive: <forward-looking warning for future modifiers>
Tested: <what was verified>
Not-tested: <known gaps in verification>
```

Use only trailers that add useful decision context.
