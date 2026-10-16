# Knowledge base

Persistent project memory, for both the user and Claude. Read the relevant file
before acting; update it after.

| File | What it holds | Update when |
|------|---------------|-------------|
| `NEXT_SESSION.md` | **Start here.** Explicit to-do list and recap for the next coding session | End of every session (rewrite it) |
| `PLAN.md` | Phased roadmap; first unchecked box = current step | A step is done or the plan changes |
| `LOG.md` | Session-by-session journal | End of every session |
| `LEARNER.md` | User's background and Rust concept progress | A concept is introduced or mastered |
| `ARCHITECTURE.md` | Target design: threads, crates, dependencies | The design changes |
| `DECISIONS.md` | Decisions with reasons, plus open questions | Any non-trivial choice is made |
| `WHEN_UNSURE.md` | Protocol for deciding what to do next | The protocol itself proves wrong |
| `GLOSSARY.md` | Audio and Rust terms | A new term comes up |
| `RESOURCES.md` | Learning links and reference projects | A useful source is found |

Start of a session: `NEXT_SESSION.md` → `LOG.md` (last entry) → `PLAN.md` (current step) → `LEARNER.md`.
