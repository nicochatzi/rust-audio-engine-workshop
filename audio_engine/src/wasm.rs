use cpal::traits::StreamTrait;
use wasm_bindgen::prelude::*;

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
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    Ok(())
}
