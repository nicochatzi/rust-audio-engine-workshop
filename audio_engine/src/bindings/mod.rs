//! Bindings for different languages
//! by default, none of these should be
//! compiled into the library. It is
//! up to the consumer to select the
//! bindings they want to use with:
//!
//! cargo build --lib --features ffi,wasm...

#[cfg(feature = "ffi")]
mod ffi;

#[cfg(feature = "pyo3")]
mod py;

#[cfg(feature = "wasm")]
mod wasm;
