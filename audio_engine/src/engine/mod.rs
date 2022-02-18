#[derive(Debug, Default)]
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

    pub fn render(&mut self, buffer: &mut [f32]) {
        let mut render_sine = || {
            let out = self.amp * self.phase.sin();
            self.phase += self.phase_inc;
            out
        };

        todo!(r#"
            Given an interleaved audio buffer with `self.num_channels` amount of channels,
            render a mono sine wave.
        "#)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_rendering_silence() {
        todo!(r#"
            Given a buffer of data with arbitrary non-zero data,
            validate that a default AudioEngine instance will render a buffer of zeros.
        "#)
    }

    fn rms(buffer: &[f32]) -> f32 {
        todo!("calculate rms value of buffer")
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

    fn expect_rms(buffer: &[f32], expected_rms: f32) {
        todo!(r#"
            Given a buffer of audio data and an expected rms value,
            check that the actual rms value is near the expected value with a tolerance of 0.001
        "#);
    }

    #[test]
    fn can_control_level_with_amp_param() {
        // generate a single cycle in the buffer.
        const BUFFER_SIZE: usize = 64;
        const SAMPLE_RATE: usize = BUFFER_SIZE;
        let mut buffer = vec![0.; BUFFER_SIZE];
        let mut engine = AudioEngine::new(SAMPLE_RATE as u32, 1);
        engine.set_freq(1.);

        todo!("Check that with an amp of 0, the buffer is all zeros");

        todo!("Check that with a non-zero amp, the rms is near the expected value");
    }
}
