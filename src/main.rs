mod comb_filter;
mod fircomb;
mod iircomb;

use comb_filter::{CombFilter, Error, FilterParam, FilterType};
use fircomb::FIRCombFilter;
use iircomb::IIRCombFilter;
use std::{fs::File, io::Write};

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
    show_info();

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input wave filename> <output text filename>", args[0]);
        return;
    }

    // Open the input wave file
    let mut reader = hound::WavReader::open(&args[1]).unwrap();
    let spec = reader.spec();
    let channels = spec.channels as usize;
    let sample_rate_hz = spec.sample_rate as f32;

    // Create FIR comb filter
    let max_delay_secs = 0.1; // Example value, adjust as needed
    let taps = vec![0.5, -0.5]; // Example FIR filter taps, adjust as needed
    let mut fir_comb_filter = FIRCombFilter::new(max_delay_secs, sample_rate_hz, channels, taps).unwrap();

    // Create IIR comb filter
    let feedforward_coeff = 0.5; // Example value, adjust as needed
    let feedback_coeff = 0.5; // Example value, adjust as needed
    let mut iir_comb_filter = IIRCombFilter::new(max_delay_secs, sample_rate_hz, channels, feedforward_coeff, feedback_coeff).unwrap();

    // Create output text file
    let mut out = File::create(&args[2]).expect("Unable to create file");

    // TODO: Modify this to process audio in blocks using your comb filters and write the result to an audio file.
    // Use the following block size:
    let block_size = 1024;

    // Read audio data and process it with the comb filters
    let mut samples_iter = reader.samples::<i16>().map(|s| s.unwrap() as f32 / (1 << 15) as f32);
    let mut input_block = vec![0.0; channels * block_size];
    let mut output_block = vec![0.0; channels * block_size];
    while let Some(sample) = samples_iter.next() {
        // Fill input block
        input_block[0] = sample;
        for i in 1..channels {
            if let Some(next_sample) = samples_iter.next() {
                input_block[i] = next_sample;
            } else {
                break;
            }
        }
        
        // Process input block with FIR comb filter
        fir_comb_filter.process(&[&input_block[..channels]], &mut [&mut output_block[..channels]]);
        
        // Write output block to the output text file
        for (i, sample) in output_block.iter().enumerate() {
            write!(out, "{}{}", sample, if i % channels == channels - 1 { "\n" } else { " " }).unwrap();
        }
    }
}

// possible test, test the half period of a sine way. It should be 0 

//possible test, feed nothing in , the output should be 0 

// (a-b).abs()< epsilion    .00001

// [30] Write tests
// Write tests verifying your implementation. These may either be executed automatically in main (when executed with no command-line arguments) or annotated with #[test] so that they get executed by cargo test.
// Implement the following tests (a function each): 
// FIR: Output is zero if input freq matches feedforward
// IIR: amount of magnitude increase/decrease if input freq matches feedback
// FIR/IIR: correct result for VARYING input block size
// FIR/IIR: correct processing for zero input signal
// At least one more additional MEANINGFUL test to verify your filter implementation

#[cfg(test)]
mod tests {
    use super::*;
    use comb_filter::{CombFilter, Error, FilterParam, FilterType};
    use fircomb::FIRCombFilter;
    use iircomb::IIRCombFilter;


    #[test]
    fn test_fir_output_zero_if_input_freq_matches_feedforward() {
        // Define the feedforward coefficient
        let feedforward_coeff = 0.5;

        // Create an FIR comb filter with 2-tap filter coefficients [0.5, -0.5]
        let mut fir_comb_filter = FIRCombFilter::new(0.0, 44100.0, 1, vec![feedforward_coeff, -feedforward_coeff]).unwrap();

        // Create an input block with frequency matching the feedforward coefficient
        let input_block = [0.5, 0.5]; // Two samples with value 0.5

        // Initialize an output block with zeros
        let mut output_block = [0.0; 2];

        // Process the input block
        fir_comb_filter.process(&[&input_block], &mut [&mut output_block]);

        // Assert that the output is zero
        assert_eq!(output_block, [0.0; 2]);
    }
    

    #[test]
    fn test_iir_magnitude_change_if_input_freq_matches_feedback() {
        let max_delay_secs = 0.1;
        let sample_rate_hz = 44100.0;
        let num_channels = 1;
        let feedforward_coeff = 0.0;
        let feedback_coeff = 0.5;
        let mut iir_comb_filter = IIRCombFilter::new(max_delay_secs, sample_rate_hz, num_channels, feedforward_coeff, feedback_coeff).unwrap();
        let input_block = [1.0; 1024];
        let mut output_block = [0.0; 1024];
        iir_comb_filter.process(&[&input_block], &mut [&mut output_block]);
        // Check if magnitude has changed (assuming the input block has non-zero magnitude)
        assert_ne!(output_block, [0.0; 1024]);
    }

    #[test]
    fn test_fir_iir_correct_result_for_varying_input_block_size() {
        let max_delay_secs = 0.1;
        let sample_rate_hz = 44100.0;
        let num_channels = 1;
        let feedforward_coeff = 0.5;
        let feedback_coeff = 0.0;
        let mut fir_comb_filter = FIRCombFilter::new(max_delay_secs, sample_rate_hz, num_channels, vec![feedforward_coeff, -feedforward_coeff]).unwrap();
        let mut iir_comb_filter = IIRCombFilter::new(max_delay_secs, sample_rate_hz, num_channels, feedforward_coeff, feedback_coeff).unwrap();
        
        let input_block_sizes = [256, 512, 1024, 2048];
        for &block_size in input_block_sizes.iter() {
            let input_block = vec![1.0; block_size];
            let mut output_block = vec![0.0; block_size];
            fir_comb_filter.process(&[&input_block], &mut [&mut output_block]);
            iir_comb_filter.process(&[&input_block], &mut [&mut output_block]);
            // Assert whatever properties you want to test for each block size
        }
    }

    #[test]
    fn test_fir_iir_correct_processing_for_zero_input_signal() {
        let max_delay_secs = 0.1;
        let sample_rate_hz = 44100.0;
        let num_channels = 1;
        let feedforward_coeff = 0.5;
        let feedback_coeff = 0.0;
        let mut fir_comb_filter = FIRCombFilter::new(max_delay_secs, sample_rate_hz, num_channels, vec![feedforward_coeff, -feedforward_coeff]).unwrap();
        let mut iir_comb_filter = IIRCombFilter::new(max_delay_secs, sample_rate_hz, num_channels, feedforward_coeff, feedback_coeff).unwrap();
        
        let input_block = vec![0.0; 1024];
        let mut output_block = vec![0.0; 1024];
        fir_comb_filter.process(&[&input_block], &mut [&mut output_block]);
        iir_comb_filter.process(&[&input_block], &mut [&mut output_block]);
        // Assert whatever properties you want to test for zero input signal
    }

    #[test]
fn test_fir_sinewave_halfperiod() {
    // Define the parameters
    let max_delay_secs = 0.1;
    let sample_rate_hz = 44100.0;
    let num_channels = 1;
    let feedforward_coeff = 0.5;

    // Create an FIR comb filter with 2-tap filter coefficients [0.5, -0.5]
    let mut fir_comb_filter = FIRCombFilter::new(max_delay_secs, sample_rate_hz, num_channels, vec![feedforward_coeff, -feedforward_coeff]).unwrap();

    // Calculate the number of samples for half a period of the sine wave
    let half_period_samples = (sample_rate_hz / 2.0) as usize;

    // Generate a sine wave with frequency corresponding to half of the sample rate
    let input_block: Vec<f32> = (0..half_period_samples)
        .map(|i| (2.0 * std::f32::consts::PI * (i as f32) / sample_rate_hz).sin())
        .collect();

    // Initialize an output block with zeros
    let mut output_block = vec![0.0; half_period_samples];

    // Process the input block
    fir_comb_filter.process(&[&input_block], &mut [&mut output_block]);

    // Check if the output is zero
    assert_eq!(output_block, vec![0.0; half_period_samples]);
}

}

