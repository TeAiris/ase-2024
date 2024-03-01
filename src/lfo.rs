pub struct LFO {
    sample_rate: f32,
    frequency: f32,
    phase: f32,
}

impl LFO {
    pub fn new(sample_rate: f32, frequency: f32) -> Self {
        LFO {
            sample_rate,
            frequency,
            phase: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn generate(&mut self, num_samples: usize) -> Vec<f32> {
        let mut output = Vec::with_capacity(num_samples);

        for _ in 0..num_samples {
            let sample = (2.0 * std::f32::consts::PI * self.phase).sin();
            self.phase += self.frequency / self.sample_rate;
            if self.phase > 1.0 {
                self.phase -= 1.0;
            }
            output.push(sample);
        }

        output
    }
}
