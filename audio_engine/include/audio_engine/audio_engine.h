#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace audio_engine {

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
struct AudioEngine
{
    float sample_time;
    uint16_t num_channels;
    float phase_inc;
    float phase;
    float amp;
};

extern "C" {

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
AudioEngine *ffi_create();

/// # Safety
///
/// It is up to the caller to call this method only
/// once and ONLY pass in a pointer that was created
/// with `ffi_create()`.
///
/// TODO: Check the pointer is null after dropping,
/// which would avoid double-free because of
/// null pointer optimisation.
void ffi_destroy(AudioEngine *engine);

/// Prepare the AudioEngine to start rendering.
/// Must be called before `ffi_render()`.
void ffi_prepare(AudioEngine *engine,
                 uint32_t sample_rate,
                 uint16_t num_channels);

/// # Safety
///
/// It is up to the caller to ensure the validity of the
/// buffer over the specified length. To avoid branching,
/// we are assuming that the pointer is valid here.
void ffi_render(AudioEngine *engine, float *buffer, uint32_t buffer_size);

void ffi_set_freq(AudioEngine *engine, float freq);

void ffi_set_amp(AudioEngine *engine, float amp);

} // extern "C"

} // namespace audio_engine
