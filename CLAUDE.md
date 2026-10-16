# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this project is

`abrust` is an Ableton-like DAW in Rust and, just as importantly, a **Rust learning project** for the user. The user is learning Rust as we go. Teaching is part of every task.

## Knowledge base: read before acting

All persistent project context lives in `knowledge/` (index: `knowledge/README.md`).

At the **start of a session**, read:
1. `knowledge/NEXT_SESSION.md`: the user's explicit to-do list and recap
2. `knowledge/LOG.md` (last entry): where we left off
3. `knowledge/PLAN.md`: the current step is the first unchecked box
4. `knowledge/LEARNER.md`: what the user already knows

When unsure what to do, or whether to ask, follow `knowledge/WHEN_UNSURE.md`.
Before re-opening a design question, check `knowledge/DECISIONS.md`.

At the **end of a session** (or after a meaningful step): rewrite `NEXT_SESSION.md` (explicit, step-by-step, written for the user), tick boxes in `PLAN.md`, append to `LOG.md`, update concept progress in `LEARNER.md`, and record any decision in `DECISIONS.md`.

## How to work with the user

- Go step by step, following `PLAN.md`. Don't jump phases or add features that aren't in the plan without asking.
- Introduce each new Rust concept before using it, in plain words with a small example. Link it to the code we're writing.
- Prefer letting the user write code with guidance; when Claude writes code, keep it short and explain every new construct.
- Explain compiler errors rather than silently fixing them; they're learning material.
- Keep code simple: no `unsafe`, no custom macros, no premature generics or abstractions.

## Non-negotiable engineering rule: the audio thread

Code that runs inside the audio callback must never allocate/free, lock, do I/O (including `println!`), or block. Communicate with it through lock-free queues/atomics; see `knowledge/ARCHITECTURE.md`. Point out violations whenever they appear.

## Commands

```sh
cargo run              # build and run
cargo check            # fast type-check
cargo test             # all tests
cargo test <name>      # tests whose name contains <name>
cargo clippy           # lints
cargo fmt              # format
```

Linux audio crates (`cpal`) need `libasound2-dev`.
