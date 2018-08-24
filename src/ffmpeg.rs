extern crate itertools;

use std::str::FromStr;
use std::process::Command;
use itertools::Itertools;
use std::ops::DerefMut;

#[derive(Debug, AsRefStr, PartialEq)]
pub enum ResultType {
    MP4,
    JPG,
}

impl FromStr for ResultType {
    type Err = ();

    fn from_str(s: &str) -> Result<ResultType, ()> {
        match s.to_lowercase().as_ref() {
            "mp4" => Ok(ResultType::MP4),
            "jpg" => Ok(ResultType::JPG),
            _ => Err(())
        }
    }
}

pub struct FfmpegBuilder {
    result_type: Option<ResultType>,
    size_x: Option<u32>,
    size_y: Option<u32>,
    pos_x: Option<u32>,
    pos_y: Option<u32>,
    record_audio: bool,
}

impl FfmpegBuilder {
    pub fn new() -> Self {
        FfmpegBuilder {
            result_type: None,
            size_x: None,
            size_y: None,
            pos_x: None,
            pos_y: None,
            record_audio: false,
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

    pub fn build(mut self) -> Command {
        if self.result_type.is_none() || self.size_y.is_none() || self.size_x.is_none() || self.pos_x.is_none() || self.pos_y.is_none() {
            panic!("FfmpegBuilder is incomplete")
        }

        let mut cmd = Command::new("ffmpeg");

        let (result_type, size_x, size_y, pos_x, pos_y) = (
            self.result_type.unwrap(),
            self.size_x.unwrap(),
            self.size_y.unwrap(),
            self.pos_x.unwrap(),
            self.pos_y.unwrap()
        );

        if result_type == ResultType::MP4 {
            cmd.args(&[
                "-hwaccel",
                "vaapi",
                "-vaapi_device",
                "/dev/dri/renderD128",
            ]);

            cmd.args(&[
                "-video_size",
                [size_x, size_y].iter().join("x").as_ref(),
                "-framerate",
                "60"
            ]);

            cmd
                .args(&[
                    "-f",
                    "x11grab"
                ])
                .args(&[
                    "-i",
                    (":0.0+".to_owned() + ([pos_x, pos_y].iter().join(",").as_ref())).as_ref()
                ]);

            if self.record_audio {
                // TODO: don't hardcode input device and interface nr
                cmd.args(&[
                    "-f",
                    "alsa",
                    "-ac",
                    "2",
                    "-i",
                    "hw:1",
                    "-acodec",
                    "copy"
                ]);
            }

            cmd.args(&[
                "-vf",
                "format=nv12,hwupload",
                "-bit_rate",
                "320k",
                "-c:v",
                "h264_vaapi",
            ]);
        }


        // finally specify output file
        cmd.args(&[
            "-y",
            "-bf",
            "0",
            "temp.mkv"
//            ("temp.".to_owned() + result_type.as_ref()).as_ref()
        ]);

        cmd
    }

}