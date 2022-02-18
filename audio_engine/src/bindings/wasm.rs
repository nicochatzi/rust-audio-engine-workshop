use cpal::traits::StreamTrait;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen]
pub struct AudioStreamHandle(cpal::Stream);

#[wasm_bindgen]
pub fn play() -> Result<AudioStreamHandle, JsValue> {
    let stream = crate::create_audio_engine_stream().map_err(|e| e.to_string())?;
    stream.play().map_err(|e| e.to_string())?;
    Ok(AudioStreamHandle(stream))
}

/// Entry point called when the WASM module is
/// loaded as well as at the start of a new thread.
///
/// https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/start.html
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub struct AudioEngine(crate::AudioEngine);

#[wasm_bindgen]
impl AudioEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(sample_rate: u32, num_channels: u16) -> Self {
        Self(crate::AudioEngine::new(sample_rate, num_channels))
    }

    pub fn set_freq(&mut self, freq: f32) {
        self.0.set_freq(freq);
    }

    pub fn set_amp(&mut self, amp: f32) {
        self.0.set_amp(amp);
    }

    pub fn prepare(&mut self, sample_rate: u32, num_channels: u16) {
        self.0.prepare(sample_rate, num_channels);
    }

    pub fn render(&mut self, buffer: &mut [f32]) {
        self.0.render(buffer);
    }
}

#[wasm_bindgen_test]
fn x() {}
