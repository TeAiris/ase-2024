use crate::{CombFilter, Error, FilterParam, FilterType};

pub struct IIRCombFilter {
    comb_filter: CombFilter,
    feedforward_coeff: f32,
    feedback_coeff: f32,
}

impl IIRCombFilter {
    pub fn new(max_delay_secs: f32, sample_rate_hz: f32, num_channels: usize, feedforward_coeff: f32, feedback_coeff: f32) -> Result<Self, Error> {
        let mut comb_filter = CombFilter::new(FilterType::IIR, max_delay_secs, sample_rate_hz, num_channels);
        comb_filter.set_param(FilterParam::Gain, 1.0)?; // Default gain
        let mut iir_comb_filter = IIRCombFilter {
            comb_filter,
            feedforward_coeff,
            feedback_coeff,
        };
        // Reset the delay line
        iir_comb_filter.comb_filter.reset();
        Ok(iir_comb_filter)
    }

    pub fn process(&mut self, input: &[&[f32]], output: &mut [&mut [f32]]) {
        self.comb_filter.process(input, output);
    }

    pub fn set_param(&mut self, param: FilterParam, value: f32) -> Result<(), Error> {
        match param {
            FilterParam::Gain => self.comb_filter.set_param(param, value),
            FilterParam::Delay => self.comb_filter.set_param(param, value),
        }
    }

    pub fn get_param(&self, param: FilterParam) -> f32 {
        self.comb_filter.get_param(param)
    }

    pub fn set_feedforward_coeff(&mut self, value: f32) {
        self.feedforward_coeff = value;
    }

    pub fn set_feedback_coeff(&mut self, value: f32) {
        self.feedback_coeff = value;
    }
}
