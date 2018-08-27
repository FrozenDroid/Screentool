#![allow(dead_code)]
extern crate itertools;
extern crate clap;
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod ffmpeg;

use itertools::Itertools;
use clap::{App, Arg};
use ffmpeg::{FfmpegBuilder, AudioDevice};

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
                .possible_values(&["mp4", "jpg", "png"])
                .case_insensitive(true)
                .default_value("png")
        )
        .arg(
            Arg::with_name("position")
                .short("p")
                .long("position")
                .value_name("POSITION")
                .help("Position of screengrab.")
                .takes_value(true)
                .default_value("0,0")
                .required(true)
        )
        .arg(
            Arg::with_name("size")
                .value_name("SIZE")
                .short("s")
                .long("size")
                .help("Size of screengrab.")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("acceleration")
                .value_name("ACCELERATION")
                .short("accel")
                .long("acceleration")
                .help("Type of hardware acceleration.")
                .possible_values(&["nvenc", "vaapi"])
                .takes_value(true)
        )
        .args(&[
            Arg::with_name("audio_backend")
                .long("audio-backend")
                .help("Specify audio backend.")
                .possible_values(&["alsa", "jack", "pulse"])
                .requires_all(&["audio_device", "audio_channels"])
                .takes_value(true)
                .required(false),
            Arg::with_name("audio_device")
                .long("audio-device")
                .help("Specify audio device identifer.")
                .takes_value(true)
                .required(false),
            Arg::with_name("audio_channels")
                .long("audio-channels")
                .help("Set amount of channels to record.")
                .possible_values(&["1", "2", "mono", "stereo"])
                .hide_default_value(true)
                .default_value("stereo")
                .required(false),
        ])
        .arg(
            Arg::with_name("output")
                .help("Path of file to output to.")
                .required(true)
                .takes_value(true)
        )
        .get_matches();

    let position_iterator = matches.value_of("position").unwrap().split(",");
    let size_iterator = matches.value_of("size").unwrap().split(",");
    if position_iterator.clone().count() != 2 || size_iterator.clone().count() != 2 {
        panic!();
    }

    let (pos_x, pos_y) = position_iterator.map(|n: &str| n.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap();

    let (size_x, size_y) = size_iterator.map(|n: &str| n.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap();
    let result_type = matches.value_of("type").unwrap().parse().unwrap();


    let mut builder = FfmpegBuilder::new(matches.value_of("output").unwrap().parse().unwrap());

    builder = builder
        .set_result_type(result_type)
        .set_position(pos_x, pos_y)
        .set_size(size_x, size_y);

    let accel_arg = matches.value_of("acceleration");

    if matches.is_present("audio_backend") {
        builder = builder.set_record_audio(true)
            .set_audio_backend(AudioDevice {
                backend: matches.value_of("audio_backend").unwrap().parse().unwrap(),
                identifier: matches.value_of("audio_device").unwrap().to_string(),
                channels: matches.value_of("audio_channels").unwrap().parse().unwrap(),
            });
    }

    if accel_arg.is_some() {
        builder = builder.set_hardware_acceleration(accel_arg.unwrap().parse().unwrap());
    }

    let mut command = builder.build().spawn().unwrap();

    std::process::exit(command.wait().unwrap().code().unwrap())
}
