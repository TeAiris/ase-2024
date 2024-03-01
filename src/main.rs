use std::{fs::File, io::{Write, BufWriter}};

mod ring_buffer;
mod vibrato;
mod lfo;

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
    let mut reader = match hound::WavReader::open(&args[1]) {
        Ok(reader) => reader,
        Err(err) => {
            eprintln!("Error opening input file: {}", err);
            return;
        }
    };
    let spec = reader.spec();
    let num_channels = spec.channels as usize; // Added to fix compilation issue

    // Open the output text file
    let out_file = match File::create(&args[2]) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error creating output file: {}", err);
            return;
        }
    };
    let mut out_text = BufWriter::new(out_file);

    // Write processing status message at the top of the output text file
    //This was used as I was debugging to figure out why it wouldn't write the text out. It is fixed now. 
    // let processing_status = if let Some(file_name) = args.get(1) {
    //     format!("Processing audio file: {}\n", file_name)
    // } else {
    //     "Processing audio file\n".to_string()
    // };
    // if let Err(err) = out_text.write_all(processing_status.as_bytes()) {
    //     eprintln!("Error writing processing status: {}", err);
    //     return;
    // }

    // Set up vibrato parameters
    let samplerate = spec.sample_rate as f32;
    let mod_freq = 5.0; // Adjust modulation frequency as needed
    let mod_depth = 0.1; // Adjust modulation depth as needed
    let delay_time_sec = 0.1; // Adjust delay time as needed
    let mut vibrato = vibrato::Vibrato::new(samplerate, mod_freq, mod_depth, delay_time_sec);

    

// Read audio data, process with vibrato, and write to the output text file
let mut samples = Vec::new(); // Accumulate samples to pass to vibrato.process()
for sample in reader.samples::<i32>() {
    let sample = match sample {
        Ok(sample) => sample as f32 / (1 << 31) as f32, // Convert sample to f32 with normalization
        Err(err) => {
            eprintln!("Error reading sample: {}", err);
            return;
        }
    };
    samples.push(sample);
    if samples.len() == num_channels {
        let processed_samples = vibrato.process(&samples); // Apply vibrato processing
        for processed_sample in processed_samples {
            if let Err(err) = write!(out_text, "{:.6} ", processed_sample) {
                eprintln!("Error writing sample to file: {}", err);
                return;
            }
        }
        if let Err(err) = out_text.write_all(b"\n") {
            eprintln!("Error writing newline to file: {}", err);
            return;
        }
        if let Err(err) = out_text.flush() {
            eprintln!("Error flushing output buffer: {}", err);
            return;
        }
        samples.clear();
    }
             }
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
