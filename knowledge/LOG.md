# Session log

Append one entry per working session, newest at the bottom. Keep entries short:
what was done, what was learned, what's next.

```
## YYYY-MM-DD: short title
**Done:** ...
**Learned:** ...
**Problems / open questions:** ...
**Next:** ...
```

---

## 2026-10-16: Project kickoff
**Done:** Created the knowledge base (`knowledge/`), roadmap (`PLAN.md`), target architecture, decision protocol, README and CLAUDE.md. No code yet.
**Environment:** Linux, rustc 1.99.1, cargo 1.99.1, git 2.53.0 installed. Directory is not yet a git repo.
**Learned:** —
**Problems / open questions:** User's programming background and preferred working style still unknown (see `LEARNER.md`).
**Next:** Phase 0, step 1: `git init`, `cargo new`, tour of `Cargo.toml` and cargo commands.

## 2026-10-16: Phase 0, steps 1–3
**Done:** `cargo init` (edition 2024), `hound 3.5.1` added. `generate_sine()` written. WAV-writing `main` written. Commits `98bf6c2`, `3ed82a6`.
**Learned:** cargo basics, `let`/`mut`, numeric types and `as`, functions with return values, `for` over ranges, `Vec`. Asked *why `&samples`* → explained borrowing vs moving, `*s` dereference, slices (`&samples[..5]`).
**Problems / open questions:** Code does not compile: typo `hound::WavWrittter` (src/main.rs:26, error E0433). Left for the user to fix as a compiler-error reading exercise. `*.wav` not yet in `.gitignore`. Background and working-style questions still unanswered.
**Next:** follow `NEXT_SESSION.md`: fix typo → hear `sine.wav` → unit tests → commit → start Phase 1 (`cpal`).
