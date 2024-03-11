//! # Vibrato
//!
//! The `Vibrato` struct provides a simple vibrato effect processor.
//!
//! ## Design Choices
//!
//! - The `Vibrato` struct consists of a delay line implemented using a ring buffer and an LFO (Low Frequency Oscillator) for modulation.
//! - Ring buffer is chosen for the delay line due to its efficient implementation for this purpose, providing constant-time access to samples.
//! - The delay line length is determined based on the desired delay time in seconds and the sampling rate.
//! - Modulation depth and modulation frequency parameters allow control over the intensity and speed of the vibrato effect.
//! - The `process` method applies vibrato effect to the input signal by modulating the delay time and mixing the delayed and original signals.
//! - Unit tests are provided to ensure the correctness of the `process` method under various scenarios.
//!
//! ## Example
//!
//! ```
//! use vibrato::Vibrato;
//!
//! // Create a new Vibrato processor
//! let samplerate = 44100.0;
//! let mod_freq = 5.0; // Hz
//! let mod_depth = 0.1;
//! let delay_time_sec = 0.1;
//! let mut vibrato = Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);
//!
//! // Process an input signal
//! let input_signal = vec![0.0, 0.25, 0.5, 0.75, 1.0];
//! let output_signal = vibrato.process(&input_signal);
//! ```
//!
//! The `output_signal` will contain the vibrato-modulated version of the `input_signal`.
//!


use crate::ring_buffer::RingBuffer;
use crate::lfo::LFO;

pub struct Vibrato {
    delay_line: RingBuffer<f32>,
    samplerate: f32,
    mod_depth: f32,
    lfo: LFO,
}

impl Vibrato {
    pub fn new(samplerate: f32, mod_freq: f32, mod_depth: f32, delay_time_sec: f32) -> Self {
        let delay_samples = (delay_time_sec * samplerate) as usize;
        let delay_line_length = delay_samples + 1;
        let delay_line = RingBuffer::new(delay_line_length);
        let lfo = LFO::new(samplerate, mod_freq);

        Vibrato {
            delay_line,
            samplerate,
            mod_depth,
            lfo,
        }
    }

    pub fn process(&mut self, input: &[f32]) -> Vec<f32> {
        let len = input.len();
        let mut output = Vec::with_capacity(len);

        let lfo_waveform = self.lfo.generate(len);

        for n in 0..len {
            let mod_signal = lfo_waveform[n] * self.mod_depth;
            let delay_time_sec = ((self.delay_line.len() as f32 - 1.0) + mod_signal) / self.samplerate;

            let delay_samples = (delay_time_sec * self.samplerate) as usize;
            if delay_samples > 0 {
                self.delay_line.set_capacity(delay_samples + 1);

                self.delay_line.push(input[n]);

                let output_sample = self.delay_line.get_frac(delay_time_sec) * 0.5 + input[n] * 0.5;
                output.push(output_sample);
            } else {
                output.push(input[n]); // No delay applied
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Verifies `Vibrato`'s `process` method with a predefined input signal.
    fn test_vibrato_process() {
        let input_signal = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        let samplerate = 44100.0;
        let mod_freq = 5.0; // Hz
        let mod_depth = 0.1;
        let delay_time_sec = 0.1;
        let mut vibrato = Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);
        let output_signal = vibrato.process(&input_signal);
        assert_eq!(output_signal.len(), input_signal.len());
    }

    #[test]
    /// Ensures `Vibrato`'s `process` method handles an empty input signal correctly.
    fn test_vibrato_process_with_no_input() {
        let samplerate = 44100.0;
        let mod_freq = 5.0;
        let mod_depth = 0.5;
        let delay_time_sec = 0.01;
        let mut vibrato = Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);
        let input: Vec<f32> = vec![];
        let output = vibrato.process(&input);
        assert_eq!(output, Vec::<f32>::new());
    }

    #[test]
    /// Checks `Vibrato`'s `process` method with a single input sample.
    fn test_vibrato_process_with_single_input() {
        let samplerate = 44100.0;
        let mod_freq = 10.0;
        let mod_depth = 0.25;
        let delay_time_sec = 0.005;
        let mut vibrato = Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);
        let input = vec![0.123];
        let output = vibrato.process(&input);
        assert_eq!(output.len(), 1);
        assert_eq!(output[0], input[0]);
    }

    #[test]
    /// Validates `Vibrato`'s `process` method with multiple input samples.
    fn test_vibrato_process_with_multiple_inputs() {
        let samplerate = 22050.0;
        let mod_freq = 2.5;
        let mod_depth = 0.75;
        let delay_time_sec = 0.02;
        let mut vibrato = Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);
        let input = vec![-0.1, 0.25, -0.35, 0.4];
        let output = vibrato.process(&input);
        assert_eq!(output.len(), input.len());
        for i in 0..input.len() {
            assert_eq!(output[i], input[i]);
        }
    }
}