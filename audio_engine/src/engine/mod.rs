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
        self.phase_inc = 2.0 * core::f32::consts::PI * freq * self.sample_time;
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