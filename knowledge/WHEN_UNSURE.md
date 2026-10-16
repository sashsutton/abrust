# When unsure: decision protocol

Use this whenever it's unclear what to do next, how to build something, or
whether to ask the user. Work through the steps in order.

## 1. Re-orient
1. Read the latest entry in `LOG.md`: what happened last session, what was promised next.
2. Find the current step in `PLAN.md` (first unchecked box).
3. Check `DECISIONS.md`: was this already decided? Don't re-open settled questions without a new reason.
4. Check `LEARNER.md`: what does the user already know? Pitch the explanation there.

## 2. Classify the uncertainty
| Kind | Example | Action |
|------|---------|--------|
| **Next step unclear** | "What do we do today?" | Take the next unchecked box in `PLAN.md`. If the step is too big, split it into smaller steps, then do the first one. |
| **Technical how-to** | "How do I share data with the audio thread?" | Check `ARCHITECTURE.md`. If it doesn't cover it, research (docs.rs, crate examples), then pick the simplest approach that respects the audio-thread rules. |
| **Design choice with real trade-offs** | GUI framework, file format, graph design | **Ask the user**: give 2–3 options, a recommendation, and what each teaches. Record the answer in `DECISIONS.md`. |
| **Scope creep** | A cool feature from a later phase | Add it to the right phase in `PLAN.md`, then return to the current step. |
| **Stuck on a bug** | Borrow checker fight, crackling audio | Reduce to the smallest reproduction; explain the compiler error in plain words to the user (this is learning material); write the insight in `LEARNER.md` or `GLOSSARY.md`. |

## 3. Default biases
- **Smallest step that runs.** Something that compiles and makes sound beats a big design that doesn't.
- **Teach before building.** Introduce a concept, then use it. Don't drop 300 lines of unfamiliar Rust on the user.
- **Simple over clever.** No `unsafe`, no macros of our own, no premature generics until the plan calls for them.
- **Respect the audio thread.** When in doubt, assume the code runs on it and follow the rules.

## 4. Close the loop
After resolving: update `PLAN.md` checkboxes, add a decision to `DECISIONS.md`
if one was made, and append to `LOG.md` at the end of the session.
