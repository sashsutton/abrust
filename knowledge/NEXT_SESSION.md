# Next session: start here

*Last updated: 2026-10-16. Rewritten at the end of every session.*

## Where you are
**Phase 0, step 3 of 4.** Your code generates a 440 Hz sine wave and *almost*
writes it to `sine.wav`, but it **doesn't compile yet**.

| Step | What | Status |
|------|------|--------|
| 1 | `cargo init`, cargo commands | ✅ done |
| 2 | `generate_sine()` function | ✅ done |
| 3 | Write `sine.wav` with `hound` | 🟡 code written, 1 compile error |
| 4 | First unit tests | ⬜ not started |

## Your to-do list (≈ 30 minutes)

### 1. Fix the compile error
Run:
```sh
cargo check
```
You'll see **error E0433: cannot find `WavWrittter` in `hound`**.
Read the error before fixing it: it tells you the *file and line*
(`src/main.rs:26`) and even suggests the fix under `help:`. Learning to read
these messages is half of learning Rust.
*(Hint: count the `t`s.)*

### 2. Run it and listen
```sh
cargo run
aplay sine.wav        # or open it in any media player
```
You should hear a 2-second beep. That's the first sound made by your DAW.

### 3. Keep `.wav` files out of git
Add one line to `.gitignore`:
```
*.wav
```

### 4. Experiment (optional but recommended)
In `main`, try changing the numbers you pass to `generate_sine`:
- frequency `220.0` (lower) or `880.0` (higher)
- amplitude `1.5`. What does it sound like? (Values above 1.0 get **clipped**.)
- **Challenge:** a chord. Generate two sines (e.g. 440 and 550 Hz) and add
  them sample by sample into a new `Vec`. Hint: loop over `0..a.len()` and
  `push(a[i] + b[i])`. Keep the total amplitude ≤ 1.0.

### 5. Step 4: write your first tests
Paste this at the **bottom** of `src/main.rs`:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sine_has_correct_length() {
        let samples = generate_sine(440.0, 1.0, 0.5);
        assert_eq!(samples.len(), 48_000);
    }

    #[test]
    fn sine_never_exceeds_amplitude() {
        let samples = generate_sine(440.0, 1.0, 0.5);
        let peak = samples.iter().fold(0.0_f32, |max, s| max.max(s.abs()));
        assert!(peak <= 0.5);
        assert!(peak > 0.49);
    }
}
```
Run `cargo test`. Both should pass. Then **break one on purpose** (e.g.
expect `47_000`) to see what a failing test looks like.

- `#[cfg(test)]`: this code only exists when running tests
- `use super::*;`: "let me use everything from the file above" (like `generate_sine`)
- `assert_eq!(a, b)`: the test fails if `a != b`

### 6. Tidy up and commit
```sh
cargo fmt
cargo clippy
git add -A
git commit -m "Phase 0 complete: sine wave to WAV + tests"
```

## Concepts to review (if anything feels fuzzy)
- **Ownership & borrowing.** `for s in &samples` *borrows* the list;
  `for s in samples` would *move* (consume) it. `*s` reads the value behind a reference.
  Re-do the experiment: remove `&`, use `samples` after the loop, read error E0382.
- **`Result` and `?`.** A function that can fail returns `Ok(...)` or `Err(...)`;
  `?` returns the error immediately if there is one.
- Reading: *The Rust Book*, chapter 3 (basics) and chapter 4 (ownership).
  Offline: `rustup doc --book`.

## After Phase 0: what's next
**Phase 1: real-time audio.** Instead of writing a file, play the sine wave
live through your speakers with the `cpal` crate, and change its pitch while
it plays. Before that session, install the audio headers:
```sh
sudo apt install libasound2-dev
```

## Two questions still open (tell Claude next time)
1. Have you programmed before (which languages)? Any music-production experience?
2. Do you like this style (you type the code, Claude explains), or prefer something else?
