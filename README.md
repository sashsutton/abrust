# abrust

An open-source digital audio workstation (DAW) written in Rust, inspired by
Ableton Live. It is also a learning project: it's built step by step so its
author learns Rust while making it.

> **Status:** planning. No code yet. See [`knowledge/PLAN.md`](knowledge/PLAN.md).

## Goals
- A real-time audio engine that never glitches (strict audio-thread rules)
- Arrangement View (timeline) and Session View (clip-launch grid)
- Audio and MIDI tracks, a built-in synth and sampler, effects and automation
- Recording, export to WAV, project save/load
- Eventually: CLAP plugin hosting

## Roadmap (summary)
| Phase | Milestone |
|-------|-----------|
| 0 | Generate a sine wave and write it to a WAV file |
| 1 | Real-time audio output with `cpal` |
| 2 | Engine core: nodes, mixer, workspace layout |
| 3 | Sample playback and transport (tempo, loop, metronome) |
| 4 | Timeline model, undo/redo, save/load |
| 5 | GUI with `egui`: transport, arrangement, mixer |
| 6 | MIDI input, synth, piano roll |
| 7 | Session View with quantized clip launching |
| 8 | Effects and automation |
| 9 | Recording |
| 10 | Export, performance, settings |
| 11 | Plugin hosting (CLAP) |

Full details: [`knowledge/PLAN.md`](knowledge/PLAN.md). Design: [`knowledge/ARCHITECTURE.md`](knowledge/ARCHITECTURE.md).

## Building
Requires a recent stable Rust toolchain (install via [rustup](https://rustup.rs)).
On Linux, audio needs the ALSA development headers:

```sh
sudo apt install libasound2-dev   # Debian/Ubuntu
cargo run
```

*(Build instructions will grow as the project does.)*

## Repository layout
- `knowledge/`: project knowledge base: plan, architecture, decisions, session log, learning notes
- `CLAUDE.md`: instructions for Claude Code sessions working on this repo

## License
To be decided (see `knowledge/DECISIONS.md`).
