# Decision log

One entry per decision that would be costly or confusing to revisit. Newest at
the bottom. Format:

```
## D-NNN: Title (YYYY-MM-DD)
**Status:** accepted | superseded by D-XXX
**Context:** why a decision was needed
**Decision:** what we chose
**Alternatives:** what we didn't choose and why
```

---

## D-001: Language and overall approach (2026-10-16)
**Status:** accepted
**Context:** The user wants an open-source, Ableton-like DAW and wants to learn Rust while building it.
**Decision:** Rust, built incrementally following `PLAN.md`. Every phase must produce something runnable and teach specific Rust concepts.
**Alternatives:** Writing a large architecture up front, rejected because the user is learning and needs fast feedback loops.

## D-002: Knowledge base location (2026-10-16)
**Status:** accepted
**Context:** Claude Code sessions don't remember previous sessions; the project needs persistent context.
**Decision:** All project knowledge lives in `knowledge/*.md`, versioned with the code. `CLAUDE.md` points to it.
**Alternatives:** Only Claude's private memory, rejected because it isn't visible to the user or other contributors.

## Open questions (to decide when we reach them)
- GUI framework: `egui` (provisional) vs `iced` vs `vizia`. Decide at Phase 5.
- Project file format: RON vs JSON. Decide at Phase 4.
- License: MIT/Apache-2.0 dual (Rust convention) vs GPL-3.0 (keeps forks open). Decide before the first public push.
