# Learner profile and Rust progress

Keeps track of what the user knows, so explanations are pitched right.
Update after every session.

## Background
- Goal: learn Rust by building a DAW.
- Prior programming experience: *unknown, ask in the first session*
- Music/audio background: *unknown, ask in the first session*
- Preferred working style: *to agree in the first session* (default: Claude explains a concept → the user writes the code with hints → Claude reviews)

## Rust concepts

`[ ]` not seen · `[~]` seen, still shaky · `[x]` comfortable

### Fundamentals
- [~] cargo: new / run / check / test / clippy / fmt (run/check/add used; test/clippy/fmt not yet)
- [~] variables, mutability, shadowing
- [~] primitive types, casting (`as`)
- [~] functions, expressions vs statements
- [~] control flow: `if`, `loop`, `while`, `for`, ranges
- [~] `Vec`, slices, arrays
- [ ] `String` vs `&str`

### Ownership
- [~] move semantics
- [~] borrowing: `&` and `&mut` (asked "why `&samples`?", a good sign)
- [ ] the borrow checker's rules
- [ ] lifetimes

### Types and abstraction
- [ ] structs, `impl`, methods
- [ ] enums, `match`, `Option`, `Result` (`Result` used in `main`)
- [~] error handling with `?`
- [ ] traits, `impl Trait`, `dyn Trait`
- [ ] generics
- [ ] iterators and closures
- [ ] modules and visibility
- [ ] derive macros (`Debug`, `Clone`, `serde`)

### Concurrency (essential for audio)
- [ ] threads
- [ ] `Send` / `Sync`
- [ ] `Arc`, `Mutex` (and why not on the audio thread)
- [ ] atomics
- [ ] lock-free queues

### Tooling
- [ ] unit tests and `cargo test`
- [ ] reading compiler errors
- [ ] reading docs on docs.rs

## Notes and insights
Things that "clicked", recurring mistakes, analogies that worked:

- *(empty)*
