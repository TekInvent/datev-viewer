# AGENTS.md

# Teki DATEV Viewer

## Purpose

This repository contains the source code for **Teki DATEV Viewer**, a lightweight, read-only desktop application for inspection and validation of DATEV EXTF files before importing them into accounting software.

Always implement according to the Product Requirements Specification (PRD). Do not introduce functionality that is not explicitly required.

---

# Core Principles

- Keep the application simple.
- Prefer maintainability over cleverness.
- Prefer deterministic behaviour over heuristics.
- Preserve strict separation between UI and business logic.
- Never implement editing of DATEV files.
- Never modify user files.

Correctness is always more important than convenience.

---

# Technology Stack

Mandatory stack:

- Tauri 2
- React
- TypeScript
- Rust

Do not replace the technology stack unless explicitly instructed.

---

# Architecture

Business logic belongs exclusively in Rust.

Rust is responsible for:

- file loading
- encoding detection
- line-ending detection
- DATEV parsing
- validation
- totals calculation
- domain model

React is responsible only for:

- presentation
- user interaction
- application state
- table rendering

Do not implement business rules in the frontend.

Architecture is intentional.

Do not move responsibilities between frontend and backend without explicit approval.

When uncertain, preserve the existing architecture.

---

# Project Structure

The following ownership rules apply.

```
src-tauri/
    Rust backend
    Tauri integration

src/
    React frontend
    UI components
    View models

datev_core/
    Parser
    Validator
    Domain model
    Totals calculation

tests/
    Automated tests
```

Keep responsibilities separated.

---

# Development Workflow

For every implementation task:

1. Understand the requirement.
2. Identify affected modules.
3. Create a short implementation plan.
4. Implement incrementally.
5. Run formatting.
6. Run linting.
7. Run tests.
8. Verify acceptance criteria.
9. Verify that no unrelated files were modified.

Avoid large, unreviewable changes.

---

# Development Environment

Assume development takes place inside the project's OrbStack development container.

Use the container for:

- dependency installation
- Rust development
- React development
- unit tests
- linting
- formatting
- static analysis

Keep the host development environment clean.

---

# Container Commands

The project provides a `docker-compose.yml` and `Dockerfile` in the repository root.

Use the following commands for all development tasks inside the container.

**Build the container image (first time or after Dockerfile changes):**
```
docker compose build
```

**Install Node dependencies:**
```
docker compose run --rm dev npm install
```

**TypeScript type-check:**
```
docker compose run --rm dev npx tsc --noEmit
```

**Check Rust compilation (all workspace crates):**
```
docker compose run --rm dev cargo check --workspace
```

**Run Rust tests:**
```
docker compose run --rm dev cargo test --workspace
```

**Run Rust linter:**
```
docker compose run --rm dev cargo clippy --workspace -- -D warnings
```

**Run Rust formatter check:**
```
docker compose run --rm dev cargo fmt --all -- --check
```

Do **not** run any of the above commands directly on the macOS host.

The only commands permitted on the host are:
- `npm run tauri dev` (Tauri desktop preview)
- `npm run tauri build` (macOS native build)

These require explicit human approval before execution.

---

# Host Machine Restrictions

Assume the host operating system is immutable unless explicitly instructed otherwise.

Do **not** perform any of the following without explicit human approval:

- install Homebrew packages
- install global npm packages
- modify shell configuration
- modify environment variables
- install software outside the repository
- modify files outside the repository

---

# Native macOS Operations

The following tasks are intentionally performed on the macOS host:

- `tauri dev`
- macOS application build
- macOS packaging
- application signing (future)
- notarization (future)
- final manual testing

Request approval before executing host-native operations.

---

# Dependency Policy

Prefer project-local dependencies.

Keep the dependency graph as small as possible.

Before introducing a new dependency:

- explain why it is needed;
- explain why existing dependencies are insufficient;
- verify that it is actively maintained;
- verify that it has a permissive license;
- avoid overlapping functionality.

Large runtime dependencies require explicit approval.

---

# UI Principles

The application is intentionally minimalistic.

Prefer:

- whitespace over decoration;
- native desktop appearance;
- simple layouts;
- readable tables;
- clear typography.

Avoid:

- unnecessary animations;
- decorative effects;
- visual clutter;
- unnecessary dialogs.

Every UI element should directly support inspection or validation.

---

# Error Handling

Never hide parsing or validation errors.

Prefer explicit diagnostics over silent recovery.

Never silently repair malformed input.

If input cannot be interpreted safely, report the problem to the user.

---

# Determinism

The application shall behave deterministically.

Given identical input files:

- parsing results shall be identical;
- validation results shall be identical;
- calculated totals shall be identical;
- displayed values shall be identical.

Never infer missing accounting information.

Never modify parsed values.

Always display exactly what exists in the source file.

---

# Coding Guidelines

- Write small, focused modules.
- Keep functions reasonably small.
- Prefer simple implementations.
- Avoid unnecessary abstractions.
- Write readable code.
- Add comments only where they improve understanding.

---

# Quality Gates

Do not consider a task complete while any of the following remain:

- compiler errors;
- compiler warnings;
- lint warnings;
- failing tests;
- TODO placeholders;
- dead code;
- commented-out obsolete code.

Leave the repository in a clean state.

---

# Testing

Implement automated tests for:

- DATEV parser;
- validation engine;
- totals calculation.

Prefer deterministic test data.

Tests must not depend on external services.

---

# Safety

The application is strictly read-only.

Never:

- overwrite user files;
- modify parsed values;
- auto-correct DATEV files;
- write back to the opened file.

---

# Git Policy

Do not:

- rewrite Git history;
- force push;
- modify unrelated files;
- rename files without justification;
- reformat unrelated code.

Keep commits focused on the requested change.

---

# Scope Control

Implement only functionality explicitly described in the PRD.

Do **not** implement:

- DATEV export generation;
- editing DATEV files;
- importing into accounting software;
- exporting reports;
- printing;
- cloud synchronization;
- online services;
- authentication;
- plugins;
- batch processing;
- account plan validation;
- tax calculations;
- document management.

If a requested task appears to extend the project scope, stop and request clarification before implementation.

---

# Definition of Success

A task is complete only when all of the following are true:

- implementation matches the PRD;
- architecture constraints are preserved;
- project builds successfully;
- automated tests pass;
- no compiler or lint warnings remain;
- no unrelated files were modified;
- the application remains strictly read-only;
- the implementation is maintainable and consistent with the existing codebase.
