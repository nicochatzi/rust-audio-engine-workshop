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
    Render,
    /// Print audio device configuration
    Info,
}

// in Rust we can return a result from the main function
// which cleans up the body of our main function nicely.
// anyhow's Result type wraps it's Error which is
// implicitly convertible from anything that implements
// std::error::Error, which should cover many cases.
fn main() -> anyhow::Result<()> {
    match App::from_args() {
        App::Play { seconds } => audio_engine::cmd::play(seconds),
        App::Render => {
            todo!(
                r#"
params:
    number of samples,  defaults to 64
    number of channels, defaults to 2
    sample rate,        defaults to 48000
            "#
            )
        }
        App::Info => {
            todo!("print the audio device config")
        }
    }
}
