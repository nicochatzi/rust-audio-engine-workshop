//! Bindings to a C ABI interface.
//! The associated header file
//! is constructed at build-time,
//! using cbindgen.

use crate::engine::AudioEngine;

/// Prepare the AudioEngine to start rendering.
/// Must be called before `ffi_render()`.
#[no_mangle]
pub extern "C" fn ffi_prepare(
    engine: Option<&mut AudioEngine>,
    sample_rate: u32,
    num_channels: u16,
) {
    if let Some(engine) = engine {
        engine.prepare(sample_rate, num_channels);
    }
}

/// # Safety
///
/// It is up to the caller to ensure the validity of the
/// buffer over the specified length. To avoid branching,
/// we are assuming that the pointer is valid here.
#[no_mangle]
pub unsafe extern "C" fn ffi_render(
    engine: Option<&mut AudioEngine>,
    buffer: *mut f32,
    buffer_size: u32,
) {
    let mut slice = core::slice::from_raw_parts_mut(buffer, buffer_size as usize);

    if let Some(engine) = engine {
        engine.render(slice);
    }
}

#[no_mangle]
pub extern "C" fn ffi_set_freq(engine: Option<&mut AudioEngine>, freq: f32) {
    if let Some(engine) = engine {
        engine.set_freq(freq);
    }
}

#[no_mangle]
pub extern "C" fn ffi_set_amp(engine: Option<&mut AudioEngine>, amp: f32) {
    if let Some(engine) = engine {
        engine.set_amp(amp);
    }
}
