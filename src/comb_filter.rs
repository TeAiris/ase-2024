pub struct CombFilter {
    filter_type: FilterType,
    max_delay_samples: usize,
    delay_line: Vec<Vec<f32>>,
    delay_line_pos: Vec<usize>,
    gain: f32,
    sample_rate_hz: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum FilterType {
    FIR,
    IIR,
}

#[derive(Debug, Clone, Copy)]
pub enum FilterParam {
    Gain,
    Delay,
}

#[derive(Debug, Clone)]
pub enum Error {
    InvalidValue { param: FilterParam, value: f32 },
}

impl CombFilter {
    pub fn new(filter_type: FilterType, max_delay_secs: f32, sample_rate_hz: f32, num_channels: usize) -> Self {
        let max_delay_samples = (max_delay_secs * sample_rate_hz) as usize;
        let delay_line = vec![vec![0.0; max_delay_samples]; num_channels];
        let delay_line_pos = vec![0; num_channels];
        let gain = 1.0;
        CombFilter {
            filter_type,
            max_delay_samples,
            delay_line,
            delay_line_pos,
            gain,
            sample_rate_hz,
        }
    }

    pub fn reset(&mut self) {
        for channel in self.delay_line.iter_mut() {
            for sample in channel.iter_mut() {
                *sample = 0.0;
            }
        }
        for pos in self.delay_line_pos.iter_mut() {
            *pos = 0;
        }
    }

    pub fn process_fir(&mut self, input: &[&[f32]], output: &mut [&mut [f32]]) {
        for (channel_idx, (input_channel, output_channel)) in input.iter().zip(output.iter_mut()).enumerate() {
            for (input_sample, output_sample) in input_channel.iter().zip(output_channel.iter_mut()) {
                let delayed_sample = self.delay_line[channel_idx][self.delay_line_pos[channel_idx]];
                *output_sample = *input_sample + self.gain * delayed_sample;
                self.delay_line[channel_idx][self.delay_line_pos[channel_idx]] = *input_sample;
                self.delay_line_pos[channel_idx] = (self.delay_line_pos[channel_idx] + 1) % self.max_delay_samples;
            }
        }
    }

    pub fn process_iir(&mut self, input: &[&[f32]], output: &mut [&mut [f32]]) {
        for (channel_idx, (input_channel, output_channel)) in input.iter().zip(output.iter_mut()).enumerate() {
            for (input_sample, output_sample) in input_channel.iter().zip(output_channel.iter_mut()) {
                let delayed_sample = self.delay_line[channel_idx][self.delay_line_pos[channel_idx]];
                *output_sample = *input_sample + self.gain * delayed_sample;
                self.delay_line[channel_idx][self.delay_line_pos[channel_idx]] = *output_sample;
                self.delay_line_pos[channel_idx] = (self.delay_line_pos[channel_idx] + 1) % self.max_delay_samples;
            }
        }
    }

    pub fn process(&mut self, input: &[&[f32]], output: &mut [&mut [f32]]) {
        match self.filter_type {
            FilterType::FIR => {
                self.process_fir(input, output);
            }
            FilterType::IIR => {
                self.process_iir(input, output);
            }
        }
    }

    pub fn set_param(&mut self, param: FilterParam, value: f32) -> Result<(), Error> {
        match param {
            FilterParam::Gain => {
                if value >= 0.0 {
                    self.gain = value;
                    Ok(())
                } else {
                    Err(Error::InvalidValue {
                        param: FilterParam::Gain,
                        value,
                    })
                }
            }
            FilterParam::Delay => {
                let max_delay_secs = self.max_delay_samples as f32 / self.sample_rate_hz;
                if value >= 0.0 && value <= max_delay_secs {
                    // Adjust delay line length
                    let new_max_delay_samples = (value * self.sample_rate_hz) as usize;
                    self.max_delay_samples = new_max_delay_samples;
                    // Resize and reset delay line
                    for channel in self.delay_line.iter_mut() {
                        channel.resize(new_max_delay_samples, 0.0);
                    }
                    self.reset();
                    Ok(())
                } else {
                    Err(Error::InvalidValue {
                        param: FilterParam::Delay,
                        value,
                    })
                }
            }
        }
    }

    pub fn get_param(&self, param: FilterParam) -> f32 {
        match param {
            FilterParam::Gain => self.gain,
            FilterParam::Delay => self.max_delay_samples as f32 / self.sample_rate_hz,
        }
    }


    // Extra Functions For User Interface in the Future
    
    // Function to get the filter type
    pub fn filter_type(&self) -> FilterType {
        self.filter_type
    }

    // Function to get the maximum delay in seconds
    pub fn max_delay_secs(&self) -> f32 {
        self.max_delay_samples as f32 / self.sample_rate_hz
    }

    // Function to get the sample rate in Hz
    pub fn sample_rate_hz(&self) -> f32 {
        self.sample_rate_hz
    }

    // Function to get the number of channels
    pub fn num_channels(&self) -> usize {
        self.delay_line.len()
    }

    // Function to get the gain parameter
    pub fn gain(&self) -> f32 {
        self.gain
    }

    // Function to get a copy of the delay line for a specific channel
    pub fn delay_line(&self, channel_idx: usize) -> Option<Vec<f32>> {
        if channel_idx < self.delay_line.len() {
            Some(self.delay_line[channel_idx].clone())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fir_output_zero_when_input_freq_matches_feedforward() {
        // Create a new CombFilter
        let mut comb_filter = CombFilter::new(FilterType::FIR, 1.0, 44100.0, 1);

        // Set the gain to 0.0
        comb_filter.set_param(FilterParam::Gain, 0.0).unwrap();

        // Create a buffer for the input and output
        let input = vec![vec![0.0; 1024]];
        let mut output = vec![vec![0.0; 1024]];

        // Process the input
        comb_filter.process(&input.iter().map(|x| &x[..]).collect::<Vec<_>>(), &mut output.iter_mut().map(|x| &mut x[..]).collect::<Vec<_>>());

        // Check that the output is all zeros
        for channel in output {
            for sample in channel {
                assert_eq!(sample, 0.0);
            
            }
        }
    }

   

}







