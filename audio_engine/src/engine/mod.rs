/// The Audio Engine data structure. These fields are
/// private since they should only be modified by
/// the engine's methods. However, in a C FFI
/// environnement the fields are exposed so that
/// the consumer knows the exact size and layout
/// of an AudioEngine instance. Due to C, the
/// fields then become public but should not be
/// modified.
///
/// The alternative would be to create an
/// opaque struct which the library would
/// handle dynamically allocating and
/// deallocating. This would hide the
/// fields but require manual memory
/// management as well as require supplying
/// an allocator which we would like to avoid
/// in a no_std environment.
#[derive(Debug, Default)]
#[repr(C)]
pub struct AudioEngine {
    sample_time: f32,
    num_channels: u16,
    phase_inc: f32,
    phase: f32,
    amp: f32,
}

impl AudioEngine {
    pub fn new(sample_rate: u32, num_channels: u16) -> Self {
        Self {
            sample_time: 1.0 / sample_rate as f32,
            num_channels,
            ..Self::default()
        }
    }

    pub fn set_freq(&mut self, freq: f32) {
        const TWO_PI: f32 = 2. * core::f32::consts::PI;
        self.phase_inc = TWO_PI * freq * self.sample_time;
    }

    pub fn set_amp(&mut self, amp: f32) {
        self.amp = amp;
    }

    pub fn prepare(&mut self, sample_rate: u32, num_channels: u16) {
        self.sample_time = 1.0 / sample_rate as f32;
        self.num_channels = num_channels;
        self.phase = 0.;
    }

    pub fn render(&mut self, buffer: &mut [f32]) {
        for frame in buffer.chunks_mut(self.num_channels as usize) {
            let output = self.phase.sin() * self.amp;
            self.phase += self.phase_inc;
            for sample in frame {
                *sample = output;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // WORKSHOP QUESTION
    fn defaults_to_rendering_silence() {
        let mut engine = AudioEngine::new(100, 1);
        let mut buffer = vec![1.; 64];
        engine.render(&mut buffer);
        assert!(buffer.iter().all(|x| *x == 0.));
    }

    // WORKSHOP STRETCH QUESTION
    fn rms(buffer: &[f32]) -> f32 {
        (buffer.iter().fold(0., |acc, x| acc + x * x) / buffer.len() as f32).sqrt()
    }

    fn sine_rms(amp: f32) -> f32 {
        amp / 2_f32.sqrt()
    }

    fn tri_rms(amp: f32) -> f32 {
        amp / 3_f32.sqrt()
    }

    fn sqr_rms(amp: f32) -> f32 {
        amp
    }

    // WORKSHOP QUESTION
    fn expect_rms(buffer: &[f32], expected_rms: f32) {
        const ERROR: f32 = 0.001;
        let actual_rms = rms(&buffer);

        // one-line option
        assert!((actual_rms - expected_rms).abs() < ERROR);

        // two asserts, but with print lhs/rhs values on failure
        assert!(actual_rms > expected_rms - ERROR);
        assert!(actual_rms > expected_rms - ERROR);
    }

    #[test]
    fn can_control_level_with_amp_param() {
        const BUFFER_SIZE: usize = 64;
        const SAMPLE_RATE: usize = BUFFER_SIZE;

        let mut buffer = vec![0.; BUFFER_SIZE];
        let mut engine = AudioEngine::new(SAMPLE_RATE as u32, 1);
        engine.set_freq(1.);

        // WORKSHOP QUESTION
        engine.set_amp(0.);
        engine.render(&mut buffer);
        assert!(buffer.iter().all(|x| *x == 0.));

        // WORKSHOP QUESTION
        const AMP: f32 = 1.;
        engine.set_amp(AMP);
        engine.render(&mut buffer);
        expect_rms(&buffer, sine_rms(AMP));
    }
}
