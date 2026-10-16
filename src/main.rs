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

fn main() {
    let samples = generate_sine(440.0, 2.0, 0.5);
    println!("Generated {} samples", samples.len());
    println!("First 5: {:?}", &samples[..5]);
}
