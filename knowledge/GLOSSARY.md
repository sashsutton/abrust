# Glossary

Audio/DAW and Rust terms as used in this project. Add terms when they come up.

## Audio / DAW
- **Sample**: one number (`f32`, usually -1.0..1.0) describing the speaker position at one instant for one channel.
- **Sample rate**: samples per second per channel (44 100, 48 000 Hz).
- **Frame**: one sample for every channel at the same instant.
- **Buffer / block size**: number of frames processed per audio callback (e.g. 256). Smaller = lower latency, higher CPU risk.
- **Latency**: delay between an action and hearing it; roughly buffer size / sample rate.
- **Interleaved**: channels stored alternately in one buffer (L R L R …).
- **Phase**: position within one cycle of a waveform (0..1 or 0..2π).
- **dB / gain**: loudness is logarithmic; `gain = 10^(dB/20)`.
- **Xrun / dropout**: the callback missed its deadline, so the user hears a click.
- **Transport**: play/stop/loop/tempo/position control.
- **BPM / beat / bar**: tempo and musical time units.
- **Clip**: a region of audio or MIDI placed on a track.
- **Session View**: Ableton's grid of clip slots launched live.
- **Arrangement View**: Ableton's linear timeline.
- **Scene**: a row of clips in Session View launched together.
- **Launch quantization**: delaying clip start to the next beat/bar so it stays in time.
- **ADSR**: Attack-Decay-Sustain-Release amplitude envelope.
- **Voice**: one sounding note in a polyphonic synth.
- **Bus**: a mix channel that sums other channels.
- **Automation**: parameter changes recorded or drawn over time.
- **Zipper noise**: crackle caused by changing a parameter in steps; fixed by smoothing.
- **CLAP / VST3**: plugin formats.

## Rust
- **Crate**: a Rust package/library. **Workspace**: several crates built together.
- **Ownership**: every value has exactly one owner; when the owner goes out of scope the value is dropped.
- **Borrow**: a reference (`&T` shared, `&mut T` exclusive) that doesn't take ownership.
- **Lifetime**: how long a borrow is valid; the compiler checks it.
- **Trait**: a set of methods a type promises to implement (like an interface).
- **`dyn Trait`**: dynamic dispatch, i.e. the concrete type is decided at runtime.
- **`Send` / `Sync`**: marker traits saying a type may be moved to / shared between threads.
- **`Arc`**: thread-safe reference-counted pointer to shared data.
- **Atomic**: a value that can be read/written from multiple threads without a lock.
- **Real-time safe**: code with bounded execution time: no allocation, no locks, no I/O.
