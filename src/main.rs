// sample code to run in terminal
// cargo run sweep.wav sweep_FIR.wav FIR 0.7 0.7
// cargo run sweep.wav sweep_IIR.wav IIR 0.7 0.7
// cargo run drumloop.wav drumloop_FIR.wav FIR 0.7 0.7
// cargo run drumloop.wav drumloop_IIR.wav IIR 0.7 0.7

mod comb_filter;


use hound::{WavReader, WavWriter};
use comb_filter::{CombFilter, FilterType, FilterParam};

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable"); 
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
    show_info();

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    let filter_type = match args[3].as_str() {
        "FIR" => FilterType::FIR,
        "IIR" => FilterType::IIR,
        _ => panic!("Invalid filter type"),
    };
    let gain: f32 = args[4].parse().unwrap();
    let delay_secs: f32 = args[5].parse().unwrap();

    // Open the input wave file and determine number of channels, sample rate, and bit depth
    let mut reader = WavReader::open(&args[1]).unwrap();
    let spec = reader.spec();
    let num_channels = spec.channels as usize;
    let sample_rate_hz = spec.sample_rate as f32;
    let bits_per_sample = spec.bits_per_sample;

    // Create a new CombFilter
    let mut comb_filter = CombFilter::new(filter_type, delay_secs, sample_rate_hz, num_channels);
    comb_filter.set_param(FilterParam::Gain, gain).unwrap();

    // Create the output wave file
    let mut writer = WavWriter::create(&args[2], spec).unwrap();

    // Process each sample with the CombFilter and write it to the output file
    for sample in reader.samples::<i32>() {
        let sample = sample.unwrap() as f32 / (1<<bits_per_sample-1) as f32;
        let mut output_sample = [0.0; 1];
        comb_filter.process(&[&[sample]], &mut [&mut output_sample]);
        let output_sample = output_sample[0];
        let max_value = (1<<(bits_per_sample-1)) - 1;
        let output_sample = (output_sample * max_value as f32).clamp(-max_value as f32, max_value as f32) as i32;
        writer.write_sample(output_sample).unwrap();
    }

    writer.finalize().unwrap();
}


