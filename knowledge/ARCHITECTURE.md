# Architecture (target design)

This file describes where we are heading. It changes as we learn; record the
reasoning for any change in `DECISIONS.md`.

## The one rule that shapes everything: the audio thread

The OS calls our audio callback every few milliseconds (e.g. 256 samples at
48 kHz ≈ 5.3 ms) and needs a full buffer back **before the deadline**. If we
are late, the user hears a click or dropout. So inside the audio callback we never:

- allocate or free memory (`Vec::push` that grows, `Box::new`, `String`, `clone()` of heap data, dropping an `Arc` that might be the last reference)
- take a lock (`Mutex`, `RwLock`): the UI thread might be holding it
- do I/O (files, network, `println!`)
- call anything with unbounded runtime

Everything else in the design exists to respect these rules.

## Threads

```
┌──────────────┐  commands (rtrb queue)   ┌───────────────┐
│  UI / main   │ ───────────────────────► │  Audio thread │ ──► sound card
│  thread      │ ◄─────────────────────── │  (cpal cb)    │
└──────┬───────┘  meters, playhead pos    └───────────────┘
       │                                         ▲
       │ spawn jobs                              │ ready-made data
       ▼                                         │ (Arc<...>)
┌──────────────┐                                 │
│ Worker       │ ── decode files, build ─────────┘
│ threads      │    new graphs, write recordings
└──────────────┘
```

- **UI thread** owns the *Project model* (tracks, clips, settings): the source of truth, freely editable, undoable, serializable.
- **Audio thread** owns the *engine state*: preallocated buffers, voices, playback cursors. It receives messages, never asks for anything.
- **Workers** do slow things (decode audio, render waveforms, disk writes) and hand finished, immutable data to the engine.
- Old data the engine no longer needs is sent *back* to a non-audio thread to be dropped there (so freeing memory never happens on the audio thread).

## Planned crate layout (Cargo workspace, from Phase 2)

```
crates/
  engine/   audio graph, transport, DSP nodes, real-time code. No UI deps.
  dsp/      (maybe) pure DSP math: oscillators, filters, envelopes. Easy to unit test.
  project/  (maybe) project model + serde + undo/redo
  app/      binary: egui GUI, wires everything together
```

Splitting only happens when it helps; start with one crate.

## Planned dependencies

| Need                  | Crate            | Notes |
|-----------------------|------------------|-------|
| Audio I/O             | `cpal`           | Cross-platform; ALSA/PipeWire/JACK on Linux |
| Lock-free queue       | `rtrb`           | Single-producer single-consumer ring buffer |
| WAV write/read        | `hound`          | Phase 0 |
| Decoding many formats | `symphonia`      | Pure Rust |
| Resampling            | `rubato`         | |
| MIDI I/O              | `midir`          | |
| GUI                   | `egui` / `eframe`| Immediate mode; easiest to learn. Revisit if we outgrow it |
| Serialization         | `serde` + `ron`  | Project files |
| Errors                | `thiserror` / `anyhow` | Library / binary |
| Plugin hosting        | `clack` (CLAP)   | Phase 11 |

All choices are provisional until recorded in `DECISIONS.md`.

## Key concepts to keep straight

- **Samples vs frames:** a frame is one sample per channel. Stereo buffer of 256 frames = 512 `f32`s (usually interleaved: L R L R …).
- **Musical time vs sample time:** the transport converts beats ↔ samples using tempo and sample rate. Store clip positions in musical time (beats), convert at playback.
- **Model vs engine state:** never let the engine read the UI's model directly. Send it a compiled snapshot.
