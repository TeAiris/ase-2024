use crate::{CombFilter, Error, FilterParam, FilterType};

pub struct FIRCombFilter {
    comb_filter: CombFilter,
    taps: Vec<f32>,
}

impl FIRCombFilter {
    pub fn new(max_delay_secs: f32, sample_rate_hz: f32, num_channels: usize, taps: Vec<f32>) -> Result<Self, Error> {
        if taps.is_empty() {
            return Err(Error::InvalidValue {
                param: FilterParam::Gain,
                value: 0.0,
            });
        }
        
        let mut comb_filter = CombFilter::new(FilterType::FIR, max_delay_secs, sample_rate_hz, num_channels);
        let gain = 1.0; // Default gain
        comb_filter.set_param(FilterParam::Gain, gain)?;
        let mut fir_comb_filter = FIRCombFilter {
            comb_filter,
            taps,
        };
        // Reset the delay line
        fir_comb_filter.comb_filter.reset();
        Ok(fir_comb_filter)
    }

    pub fn process(&mut self, input: &[&[f32]], output: &mut [&mut [f32]]) {
        self.comb_filter.process(input, output);
    }

    pub fn set_param(&mut self, param: FilterParam, value: f32) -> Result<(), Error> {
        self.comb_filter.set_param(param, value)
    }

    pub fn get_param(&self, param: FilterParam) -> f32 {
        self.comb_filter.get_param(param)
    }
}
