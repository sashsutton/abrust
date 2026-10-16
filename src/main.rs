use std::f32::consts::TAU; // TAU = 2PI

const SAMPLE_RATE: u32 = 48_000;

fn generate_sine(frequency: f32, seconds: f32, amplitude: f32) -> Vec<f32> {
    let num_samples = (SAMPLE_RATE as f32 * seconds) as usize;
    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / SAMPLE_RATE as f32; // time in seconds
        samples.push(amplitude * (TAU * frequency * t).sin());
    }
    samples
}

fn main() -> Result<(), hound::Error> {
    let samples = generate_sine(440.0, 2.0, 0.5);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec)?;

    for s in &samples {
        writer.write_sample(*s)?;
    }
    writer.finalize()?;

    println!("Wrote sine.wav");
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sine_has_correct_length() {
        let samples = generate_sine(440.0, 1.0, 0.5);
        assert_eq!(samples.len(), 48_000);
    }

    #[test]
    fn sine_never_exceeds_amplit
        let samples = generate_sine(440.0, 1.0, 0.5);
        let peak = samples.iter(max(s.abs()));
        assert!(peak <= 0.5);
        assert!(peak > 0.49);
    }
}
