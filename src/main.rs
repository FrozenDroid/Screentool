extern crate clap;
use clap::{App};
use clap::Arg;
mod ffmpeg;

fn main() {
    let matches = App::new("Screentool")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("type")
                .short("t")
                .long("type")
                .value_name("TYPE")
                .help("Type of screengrab.")
                .takes_value(true)
                .possible_values(&["mp4", "gif", "jpg"])
                .case_insensitive(true)
                .default_value("jpg")
        )
        .get_matches();

    let builder = ffmpeg::FfmpegBuilder::new();
    builder.set_result_type(
        matches.value_of("type").unwrap().parse::<ffmpeg::ResultType>().unwrap()
    );

}

