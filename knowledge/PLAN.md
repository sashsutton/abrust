# Roadmap

The goal is an open-source DAW in Rust, modeled on Ableton Live, built in small
steps so the author learns Rust along the way. Each phase ends with something
that **runs and can be heard or seen**, and introduces a handful of Rust
concepts. We don't move on until those concepts make sense.

Status markers: `[ ]` not started · `[~]` in progress · `[x]` done.
The **current step** is always the first unchecked box. Update this file as
steps complete, and log the session in `LOG.md`.

---

## Phase 0: Toolchain and first sound (offline)
Goal: make a sound without touching real-time audio yet.

- [ ] `git init`, `cargo new`, understand `Cargo.toml`, `cargo run`, `cargo check`, `cargo clippy`, `cargo fmt`
- [ ] Rust basics: `let`/`mut`, primitive types (`f32`, `usize`), functions, `for` loops, `Vec`
- [ ] Generate a 440 Hz sine wave into a `Vec<f32>` (learn: sample rate, phase, amplitude)
- [ ] Write it to a `.wav` file with the `hound` crate (learn: adding dependencies, `Result`, `?`)
- [ ] Our first unit test: check that the generated buffer has the right length and peak

**Rust concepts:** cargo, variables, mutability, types, functions, loops, `Vec`, crates, basic `Result`.

## Phase 1: Real-time audio output
Goal: hear a sine wave through the speakers live, and change its pitch while it plays.

- [ ] Open the default output device with `cpal`; print device name, sample rate, channel count
- [ ] Play a sine wave from the audio callback
- [ ] Understand the **audio thread rules** (see `ARCHITECTURE.md`): no allocation, no locks, no blocking
- [ ] Change frequency from the main thread using an atomic (`AtomicU32` + `f32::to_bits`)
- [ ] Replace the atomic with a lock-free message queue (`rtrb`) carrying commands

**Rust concepts:** ownership and borrowing, closures, `move`, threads, `Send`/`Sync`, `Arc`, atomics, enums with data, `match`.

## Phase 2: Engine core: nodes and mixing
Goal: a small audio graph: several sources → gain/pan → master.

- [ ] Define an `AudioNode`-style trait (`process(&mut self, buffer)`)
- [ ] Implement oscillator, gain and pan nodes
- [ ] A mixer that sums tracks into a master bus; a master volume and a peak meter
- [ ] Turn the project into a **Cargo workspace** (`crates/engine`, `crates/app`) so the engine knows nothing about UI
- [ ] Unit tests for DSP building blocks

**Rust concepts:** structs and `impl`, traits, `Box<dyn Trait>` vs generics, modules, visibility (`pub`), workspaces, slices (`&mut [f32]`).

## Phase 3: Sample playback and transport
Goal: load an audio file and play it in time with a tempo.

- [ ] Decode WAV/FLAC/MP3 with `symphonia` (on a non-audio thread!)
- [ ] Resample to the device rate if needed
- [ ] Transport: play / stop / loop, tempo (BPM), position in samples ↔ beats ↔ bars
- [ ] A metronome click
- [ ] Proper error types (`thiserror` in the engine, `anyhow` in the app)

**Rust concepts:** error handling in depth, `Option`, iterators, lifetimes (first real encounter), sharing loaded data with `Arc<[f32]>`.

## Phase 4: Timeline (Arrangement model)
Goal: multiple tracks with clips placed on a timeline, played back sample-accurately.

- [ ] Project model: `Project` → `Track` → `Clip` (separate from engine runtime state)
- [ ] Sample-accurate clip scheduling inside the audio callback
- [ ] Engine receives a new "playback snapshot" when the project changes (no locks)
- [ ] Undo/redo with a command pattern
- [ ] Save/load projects with `serde` (RON or JSON)

**Rust concepts:** data modeling with enums/structs, `Clone` vs move, `serde` derive macros, collections (`HashMap`, `BTreeMap`), IDs vs references.

## Phase 5: GUI shell
Goal: a window with a transport bar, an arrangement view and a mixer.

- [ ] Window with `egui`/`eframe`; transport bar wired to the engine
- [ ] Arrangement view: tracks, clips, playhead, drag clips
- [ ] Mixer view: faders, pan, meters (meter values flow engine → UI via a queue)
- [ ] Load audio files by drag-and-drop or file dialog

**Rust concepts:** immediate-mode UI, application state ownership, two-way communication between threads, borrow-checker patterns in UI code.

## Phase 6: MIDI and instruments
Goal: play a built-in synth from a MIDI keyboard and from MIDI clips.

- [ ] MIDI input with `midir`
- [ ] A polyphonic subtractive synth: oscillators, ADSR envelope, filter, voice allocation
- [ ] MIDI clips on the timeline and a basic piano roll
- [ ] A simple sampler instrument

**Rust concepts:** more traits, fixed-capacity data structures (no allocation on the audio thread), `const` generics, bit manipulation for MIDI parsing.

## Phase 7: Session View (Ableton's signature)
Goal: a clip-launch grid with scenes and quantized launching.

- [ ] Clip slot grid per track; scenes
- [ ] Launch quantization (next beat / bar)
- [ ] Follow actions (stretch goal)
- [ ] Switching between Session and Arrangement

## Phase 8: Effects and automation
- [ ] Parameter smoothing (no zipper noise)
- [ ] Built-in effects: EQ (biquads), delay, reverb, compressor
- [ ] Device chain per track
- [ ] Automation lanes, recorded and drawn

## Phase 9: Recording
- [ ] Audio input via `cpal`; recording to disk on a separate thread
- [ ] Latency compensation
- [ ] MIDI recording, overdub, count-in

## Phase 10: Export and polish
- [ ] Offline render / bounce to WAV (reuse the engine without a device)
- [ ] Performance profiling; multithreaded track processing
- [ ] Settings: audio device selection, buffer size

## Phase 11: Plugins
- [ ] Host CLAP plugins (e.g. via `clack`)
- [ ] Explore VST3 hosting (harder; licensing to check)
- [ ] Optional: ship our own effects as plugins with `nih-plug`

## Later / wishlist
Warping and time-stretching, audio-to-MIDI, groove, freeze/flatten, sidechain,
Max-for-Live-style scripting.
