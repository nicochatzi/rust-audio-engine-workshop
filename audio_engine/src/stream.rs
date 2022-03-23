//! A set of top level commands the consumer
//! can call to run the engine. Typically,
//! these will be used in an environnement
//! where the caller does not have control
//! over the audio host and requires the library
//! to handle streaming audio to the host.
use crate::engine::*;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

/// Get the default audio device and configuration that
/// will be used by the `play()` method.
pub fn get_audio_device_and_config() -> anyhow::Result<(cpal::Device, cpal::SupportedStreamConfig)>
{
    // A cpal::Host provides access to the available audio devices on the system.
    let host = cpal::default_host();

    // A cpal::Device is an audio device that may have any number of input and output streams.
    let device = host
        .default_output_device()
        .expect("no output device available");

    // A cpal::SupportedStreamConfig has the output configuration,
    // contains: sample rate, sample type, buffer size...
    let supported_config = device.default_output_config()?;

    Ok((device, supported_config))
}

pub(crate) fn create_audio_engine_stream() -> anyhow::Result<cpal::Stream> {
    let (device, supported_config) = get_audio_device_and_config()?;

    // We will only accept a configuration that uses f32 sample type
    assert!(matches!(
        supported_config.sample_format(),
        cpal::SampleFormat::F32
    ));

    // Get the currently used output configuration
    let config = supported_config.config();

    let mut engine = AudioEngine::new(config.sample_rate.0, config.channels);
    engine.set_freq(220.);
    engine.set_amp(0.7);

    // A cpal::Stream is an open flow of audio data.
    let stream = device.build_output_stream(
        &config,
        // Audio callback receives a mutable audio buffer to process
        // Any variable in the outer scope will be moved into this closure.
        move |buf: &mut [f32], _: &cpal::OutputCallbackInfo| engine.render(buf),
        // Error callback receives a cpal::StreamError
        |error| eprintln!("an error occurred on stream: {}", error),
    )?;

    Ok(stream)
}

/// Run the audio engine for a given amount of time, in seconds.
/// 0 seconds runs the engine forever.
pub fn play(num_seconds: u64) -> anyhow::Result<()> {
    let stream = create_audio_engine_stream()?;

    stream.play()?;

    wait(num_seconds);

    Ok(())
}

/// Render a block of audio at a fixed frequency and amplitude
pub fn render(num_samples: usize, channels: usize, sample_rate: usize) -> Vec<f32> {
    todo!()
}

/// pauses the thread for N seconds
/// if N==0 it loops forever.
fn wait(num_seconds: u64) {
    todo!(r#"
        hint: use can use std::thread::sleep() since
        the audio processing is happening on another thread
    "#)
}
