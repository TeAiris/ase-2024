use std::{fs::File, io::{Write, BufWriter}};
use hound::{WavReader,WavWriter, WavSpec};

mod ring_buffer;
mod vibrato;
mod lfo;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn print_usage() {
    eprintln!("Usage: ./executable <input wave filename> <output wave filename> <modulation frequency> <modulation depth> <delay time>");
}

fn main() {
    show_info();

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 6 {
        print_usage();
        return;
    }

    // Open the input wave file
    let mut reader = match WavReader::open(&args[1]) {
        Ok(reader) => reader,
        Err(err) => {
            eprintln!("Error opening input file: {}", err);
            return;
        }
    };
    let spec = reader.spec();
    let num_channels = spec.channels as usize;

    // Open the output WAV file
    let out_file = match File::create(&args[2]) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating output file: {}", err);
            return;
        }
    };

    // Prepare WAV file specifications
    let wav_spec = WavSpec {
        channels: num_channels as u16,
        sample_rate: spec.sample_rate,
        bits_per_sample: spec.bits_per_sample as u16,
        sample_format: hound::SampleFormat::Int,
    };

    // Create a WavWriter with the specified specifications
    let mut wav_writer = match WavWriter::new(out_file, wav_spec) {
        Ok(writer) => writer,
        Err(err) => {
            eprintln!("Error creating WAV writer: {}", err);
            return;
        }
    };

    // Set up vibrato parameters from command-line arguments
    let samplerate = spec.sample_rate as f32;
    let mod_freq = match args[3].parse::<f32>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error parsing modulation frequency");
            return;
        }
    };
    let mod_depth = match args[4].parse::<f32>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error parsing modulation depth");
            return;
        }
    };
    let delay_time_sec = match args[5].parse::<f32>() {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error parsing delay time");
            return;
        }
    };
    let mut vibrato = vibrato::Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);

    // Process audio data and write to the output WAV file
    let mut samples = Vec::new();
    for sample in reader.samples::<i32>() {
        let sample = match sample {
            Ok(sample) => sample as f32 / (1 << 31) as f32,
            Err(err) => {
                eprintln!("Error reading sample: {}", err);
                return;
            }
        };
        samples.push(sample);
        if samples.len() == num_channels {
            let processed_samples = vibrato.process(&samples);
            for processed_sample in processed_samples {
                // Convert processed sample back to i32 before writing to the WAV file
                let processed_sample_i32 = (processed_sample * (1 << 31) as f32) as i32;
                if let Err(err) = wav_writer.write_sample(processed_sample_i32) {
                    eprintln!("Error writing sample to file: {}", err);
                    return;
                }
            }
            samples.clear();
        }
    }

    // Finish writing to the WAV file
    if let Err(err) = wav_writer.finalize() {
        eprintln!("Error finalizing WAV file: {}", err);
        return;
    }

    eprintln!("Processing completed successfully!");
}



#[cfg(test)]
mod tests {
    use super::*;
    use super::vibrato::Vibrato;

    #[test]
    fn test_vibrato_delayed_input_when_mod_amplitude_is_zero() {
        let samplerate = 44100.0;
        let mod_freq = 5.0;
        let mod_depth = 0.0; // Modulation amplitude is zero
        let delay_time_sec = 0.1;
        let mut vibrato = Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);

        // Prepare input signal (e.g., delayed input)
        let input_signal = vec![0.0, 0.1, 0.2, 0.3, 0.4];
        let expected_output = input_signal.clone(); // Expected output is the same as input

        // Process input signal
        let output_signal = vibrato.process(&input_signal);

        // Compare output with expected output
        assert_eq!(output_signal, expected_output);
    }

    #[test]
    fn test_vibrato_dc_input_results_in_dc_output() {
        let samplerate = 44100.0;
        let mod_freq = 5.0;
        let mod_depth = 0.5; // Non-zero modulation amplitude
        let delay_time_sec = 0.1;
        let mut vibrato = Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);

        // Prepare input signal (DC input)
        let input_signal = vec![0.5; 5]; // All samples are the same DC value
        let expected_output = vec![0.5; 5]; // Expected output should be the same DC value

        // Process input signal
        let output_signal = vibrato.process(&input_signal);

        // Compare output with expected output
        assert_eq!(output_signal, expected_output);
    }

    #[test]
    fn test_vibrato_varying_input_block_size() {
        let samplerate = 44100.0;
        let mod_freq = 5.0;
        let mod_depth = 0.5;
        let delay_time_sec = 0.1;
        let mut vibrato = Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);

        // Prepare input signal with different block sizes
        let input_signals = vec![
            vec![0.1, 0.2, 0.3],
            vec![0.4, 0.5, 0.6, 0.7],
            vec![0.8, 0.9],
        ];
        let expected_outputs = vec![
            vec![0.1, 0.2, 0.3],
            vec![0.4, 0.5, 0.6, 0.7],
            vec![0.8, 0.9],
        ];

        // Process each input signal
        for (input_signal, expected_output) in input_signals.iter().zip(expected_outputs.iter()) {
            let output_signal = vibrato.process(&input_signal);
            assert_eq!(output_signal, *expected_output);
        }
    }

    #[test]
    fn test_vibrato_zero_input_signal() {
        let samplerate = 44100.0;
        let mod_freq = 5.0;
        let mod_depth = 0.5;
        let delay_time_sec = 0.1;
        let mut vibrato = Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);

        // Prepare zero input signal
        let input_signal = vec![0.0; 5];
        let expected_output = vec![0.0; 5]; // Expected output should also be zero

        // Process input signal
        let output_signal = vibrato.process(&input_signal);

        // Compare output with expected output
        assert_eq!(output_signal, expected_output);
    }
    #[test]
    fn test_vibrato_negative_modulation_depth() {
        let samplerate = 44100.0;
        let mod_freq = 5.0;
        let mod_depth = -0.5; // Negative modulation depth
        let delay_time_sec = 0.1;
        let mut vibrato = Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);
    
        // Prepare input signal
        let input_signal = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        // Expected output should be the original input signal because negative modulation depth
        // would result in no modulation effect
        let expected_output = input_signal.clone();
    
        // Process input signal
        let output_signal = vibrato.process(&input_signal);
    
        // Compare output with expected output
        assert_eq!(output_signal, expected_output);
    }
    

}
