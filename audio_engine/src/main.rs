use cpal::traits::DeviceTrait;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "audio-engine",
    about = "Audio Engine core CLI to quickly test the engine"
)]
enum App {
    /// Run the audio engine for a given amount of seconds, 0s play forever
    Play {
        /// Amount of seconds to play
        #[structopt(short, long, default_value = "2")]
        seconds: u64,
    },
    /// Render a fixed amount of samples and print to terminal
    Render {
        /// Number of samples to render
        #[structopt(long, default_value = "64")]
        samples: usize,
        /// Number of channels to render
        #[structopt(long, default_value = "2")]
        channels: usize,
        /// Sample rate to render at
        #[structopt(long, default_value = "48000")]
        sample_rate: usize,
    },
    /// Print audio and midi configuration
    Info,
}

// in Rust we can return a result from the main function
// which cleans up the body of our main function nicely.
// anyhow's Result type wraps it's Error which is
// implicitly convertible from anything that implements
// std::error::Error, which should cover many cases.
fn main() -> anyhow::Result<()> {
    match App::from_args() {
        App::Play { seconds } => audio_engine::play(seconds),
        App::Render {
            samples,
            channels,
            sample_rate,
        } => {
            let buffer = audio_engine::render(samples, channels, sample_rate);
            for frame in buffer.chunks(channels) {
                for sample in frame {
                    print!("{},\t", sample);
                }
                println!();
            }
            Ok(())
        }
        App::Info => {
            let (device, supported_config) = audio_engine::get_audio_device_and_config()?;
            println!("Output device: {}", device.name()?);
            println!("Output config: {:?}", supported_config);
            Ok(())
        }
    }
}
