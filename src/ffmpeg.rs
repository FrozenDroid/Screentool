extern crate itertools;

use std::str::FromStr;
use std::process::{Command, Stdio};
use itertools::Itertools;

#[derive(Debug, PartialEq)]
pub enum ResultType {
    MP4,
    JPG,
    PNG,
}

#[derive(PartialEq)]
pub enum HwAccelType {
    VAAPI,
    NVENC,
}

#[derive(Debug, AsRefStr, PartialEq)]
pub enum AudioBackend {
    PULSE,
    ALSA,
    JACK,
}

impl FromStr for HwAccelType {
    type Err = ();

    fn from_str(s: &str) -> Result<HwAccelType, ()> {
        match s.to_lowercase().as_ref() {
            "nvenc" => Ok(HwAccelType::NVENC),
            "vaapi" => Ok(HwAccelType::VAAPI),
            _ => Err(()),
        }
    }
}

impl FromStr for ResultType {
    type Err = ();

    fn from_str(s: &str) -> Result<ResultType, ()> {
        match s.to_lowercase().as_ref() {
            "mp4" => Ok(ResultType::MP4),
            "jpg" => Ok(ResultType::JPG),
            "png" => Ok(ResultType::PNG),
            _ => Err(()),
        }
    }
}

impl FromStr for AudioBackend {
    type Err = ();

    fn from_str(s: &str) -> Result<AudioBackend, ()> {
        match s.to_lowercase().as_ref() {
            "pulse" => Ok(AudioBackend::PULSE),
            "jack" => Ok(AudioBackend::JACK),
            "alsa" => Ok(AudioBackend::ALSA),
            _ => Err(()),
        }
    }
}

impl FromStr for AudioChannels {
    type Err = ();

    fn from_str(s: &str) -> Result<AudioChannels, ()> {
        match s.to_lowercase().as_ref() {
            "mono" => Ok(AudioChannels::MONO),
            "1" => Ok(AudioChannels::MONO),
            "stereo" => Ok(AudioChannels::STEREO),
            "2" => Ok(AudioChannels::STEREO),
            _ => Err(()),
        }
    }
}


impl FromStr for Output {
    type Err = ();

    fn from_str(s: &str) -> Result<Output, ()> {
        match s.to_lowercase().as_ref() {
            "-" => Ok(Output::STREAM),
            _ => Ok(Output::FILE(s.to_string())),
        }
    }
}


pub enum AudioChannels {
    MONO,
    STEREO
}

impl Into<String> for AudioChannels {
    fn into(self) -> String {
        match self {
            AudioChannels::MONO => "1".to_string(),
            AudioChannels::STEREO => "2".to_string(),
        }
    }
}

impl Into<String> for ResultType {
    fn into(self) -> String {
        match self {
            ResultType::JPG => "mjpeg".to_string(),
            ResultType::PNG => "apng".to_string(),
            ResultType::MP4 => "mp4".to_string(),
        }
    }
}

pub struct AudioDevice {
    pub backend: AudioBackend,
    pub identifier: String,
    pub channels: AudioChannels,
}

#[derive(PartialEq)]
pub enum Output {
    STREAM,
    FILE(String),
}

pub struct FfmpegBuilder {
    result_type: Option<ResultType>,
    size_x: Option<u32>,
    size_y: Option<u32>,
    pos_x: Option<u32>,
    pos_y: Option<u32>,
    record_audio: bool,
    hw_accel: Option<HwAccelType>,
    audio_device: Option<AudioDevice>,
    output: Output,
}

impl FfmpegBuilder {

    pub fn new(output: Output) -> Self {
        FfmpegBuilder {
            result_type: None,
            size_x: None,
            size_y: None,
            pos_x: None,
            pos_y: None,
            record_audio: false,
            hw_accel: None,
            audio_device: None,
            output
        }
    }

    pub fn set_result_type(self, result_type: ResultType) -> Self {
        Self {
            result_type: Some(result_type),
            ..self
        }
    }

    pub fn set_position(self, pos_x: u32, pos_y: u32) -> Self {
        Self {
            pos_x: Some(pos_x),
            pos_y: Some(pos_y),
            ..self
        }
    }

    pub fn set_record_audio(self, record_audio: bool) -> Self {
        Self {
            record_audio,
            ..self
        }
    }

    pub fn set_size(self, size_x: u32, size_y: u32) -> Self {
        Self {
            size_x: Some(size_x),
            size_y: Some(size_y),
            ..self
        }
    }

    pub fn set_hardware_acceleration(self, hw_accel: HwAccelType) -> Self {
        Self {
            hw_accel: Some(hw_accel),
            ..self
        }
    }

    pub fn set_audio_backend(self, audio_device: AudioDevice) -> Self {
        Self {
            audio_device: Some(audio_device),
            ..self
        }
    }

    pub fn build(self) -> Command {
        if self.result_type.is_none() || self.size_y.is_none() || self.size_x.is_none() || self.pos_x.is_none() || self.pos_y.is_none() {
            panic!("FfmpegBuilder is incomplete")
        }

        let mut cmd = Command::new("ffmpeg");

        let (result_type, size_x, size_y, pos_x, pos_y) = (
            self.result_type.unwrap(),
            self.size_x.unwrap(),
            self.size_y.unwrap(),
            self.pos_x.unwrap(),
            self.pos_y.unwrap(),
        );

        let position_str = ":0.0+".to_owned() + ([pos_x, pos_y].iter().join(",").as_ref());

        let std_flags = &[
            "-loglevel",
            "error",
            "-f",
            "x11grab",
            "-i",
            position_str.as_ref()
        ];

        let framerate = if result_type.eq(&ResultType::MP4) { 60 } else { 1 };

        cmd.args(&[
            "-video_size",
            [size_x, size_y].iter().join("x").as_ref(),
            "-framerate",
            framerate.to_string().as_ref()
        ]);

        cmd.args(std_flags);

        if result_type.eq(&ResultType::MP4) {

            if self.record_audio {
                let audio_device = self.audio_device.unwrap();
                let channels: String = audio_device.channels.into();
                // TODO: don't hardcode input device and interface nr
                cmd.args(&[
                    "-f",
                    audio_device.backend.as_ref().to_string().to_lowercase().as_ref(),
                    "-ac",
                    channels.as_ref(),
                    "-i",
                    audio_device.identifier.as_ref()
                ]);
            }

            if self.hw_accel.eq(&Some(HwAccelType::VAAPI)) {
                cmd.args(&[
                    "-hwaccel",
                    "vaapi",
                    "-vaapi_device",
                    "/dev/dri/renderD128",
                ]);
            }

            if self.hw_accel.eq(&Some(HwAccelType::VAAPI)) {
                cmd.args(&[
                    "-vf",
                    "format=nv12,hwupload",
                    "-bit_rate",
                    "320k",
                    "-c:v",
                    "h264_vaapi",
                ]);
            }

            if self.hw_accel.eq(&Some(HwAccelType::NVENC)) {
                cmd.args(&[
                    "-vcodec",
                    "h264_nvenc",
                ]);
            }

        } else if result_type.eq(&ResultType::JPG) || result_type.eq(&ResultType::PNG) {
            // Set amount of frames to record to just one
            cmd.args(&[
                "-vframes",
                "1",
            ]);
        }

//        println!("{:?}", result_type.as_ref().to_string().to_lowercase());

        let result_type: String = result_type.into();
        // finally specify output file
        cmd.args(&[
            "-y",
            "-bf",
            "0",
            "-f",
//            "image2pipe"
            result_type.as_ref()
//            result_type.as_ref().to_lowercase().as_ref()
        ]);

        match self.output {
            Output::FILE(path) => cmd.arg(path),
            Output::STREAM => cmd.args(&[
                "-"
            ]),
        };

        cmd.stdout(Stdio::inherit());

        cmd
    }

}
