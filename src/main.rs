#![allow(dead_code)]
extern crate itertools;
extern crate clap;
extern crate strum;
#[macro_use]
extern crate strum_macros;

mod ffmpeg;

use itertools::Itertools;
use clap::{App, Arg};
use ffmpeg::{FfmpegBuilder};
use std::fmt::Debug;
use ffmpeg::HwAccelType;


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
                .default_value("jpg")
        )
        .arg(
            Arg::with_name("position")
                .short("p")
                .long("position")
                .value_name("POSITION")
                .help("Position of screengrab")
                .takes_value(true)
                .default_value("0,0")
                .required(true)
        )
        .arg(
            Arg::with_name("size")
                .value_name("SIZE")
                .short("s")
                .long("size")
                .help("Size of screengrab")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("acceleration")
                .value_name("ACCELERATION")
                .short("accel")
                .long("acceleration")
                .help("Type of hardware acceleration")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("audio")
                .long("enable-audio")
                .help("Record audio")
                .default_value("true")
                .takes_value(false)
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


    let mut builder = FfmpegBuilder::new();

    builder = builder
        .set_result_type(result_type)
        .set_position(pos_x, pos_y)
        .set_size(size_x, size_y)
        .set_record_audio(matches.is_present("audio"));

    let accel_arg = matches.value_of("acceleration");

    if accel_arg.is_some() {
        builder = builder.set_hardware_acceleration(accel_arg.unwrap().parse().unwrap());
    }

    let mut command = builder.build();

    println!("{:?}\n", command);
    println!("{:?}", command.output());
}
