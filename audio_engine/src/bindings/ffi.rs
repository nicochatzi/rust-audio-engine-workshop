//! Bindings to a C ABI interface.
//! The associated header file
//! is constructed at build-time,
//! using cbindgen.

use crate::engine::*;

/// # Safety
///
/// This heap allocates, and leaks, a new AudioEngine object.
/// It is up to the caller to call `ffi_destroy()` only once
/// on the pointer.
///
/// We could also return a stack allocated AudioEngine here
/// though we would need to expose the content of the engine
/// since we would no longer be able to use AudioEngine as an
/// opaque type.
///
/// https://github.com/rust-lang/rust/issues/66220
///
/// TODO: this is not used anymore since we can create on stack
#[no_mangle]
pub unsafe extern "C" fn ffi_create() -> *mut AudioEngine {
    let engine = AudioEngine::default();
    Box::into_raw(Box::new(engine))
}

/// # Safety
///
/// It is up to the caller to call this method only
/// once and ONLY pass in a pointer that was created
/// with `ffi_create()`.
///
/// TODO: Check the pointer is null after dropping,
/// which would avoid double-free because of
/// null pointer optimisation.
#[no_mangle]
pub unsafe extern "C" fn ffi_destroy(engine: Option<&mut AudioEngine>) {
    if let Some(engine) = engine {
        // Explicitly drop the heap allocated box.
        // Technically not required, just nice to
        // be explicit about the intention here.
        drop(Box::from_raw(engine));
    }
}

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
    let slice = core::slice::from_raw_parts_mut(buffer, buffer_size as usize);
    engine.render(slice);
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
