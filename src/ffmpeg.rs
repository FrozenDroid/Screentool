use std::str::FromStr;

#[derive(Debug)]
pub enum ResultType {
    MP4,
    GIF,
    JPG,
}

impl FromStr for ResultType {
    type Err = ();

    fn from_str(s: &str) -> Result<ResultType, ()> {
        match s.to_lowercase().as_ref() {
            "mp4" => Ok(ResultType::MP4),
            "gif" => Ok(ResultType::GIF),
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
}

impl FfmpegBuilder {
    pub fn new() -> Self {
        FfmpegBuilder {
            result_type: None,
            size_x: None,
            size_y: None,
            pos_x: None,
            pos_y: None,
        }
    }

    pub fn set_result_type(self, result_type: ResultType) -> Self {
        Self {
            result_type: Some(result_type),
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
}