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
    engine: &mut AudioEngine,
    buffer: *mut f32,
    buffer_size: u32,
) {
    todo!(r"
        Given an AudioEngine reference, we want to render some audio
        directly into the buffer passed in this function.

        hint:
            we want to create a Rust 'slice' from the raw *mut f32 pointer.
            https://doc.rust-lang.org/core/slice/index.html#functions
    ");
}

// TODO:
//     Write the C FFI function to set the frequency on an AudioEngine instance

// TODO:
//     Write the C FFI function to set the amplitude on an AudioEngine instance
