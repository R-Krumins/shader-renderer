use crate::shader;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub enum OutputFormat {
    MP4,
    GIF,
}

pub struct Config {
    pub width: usize,
    pub height: usize,
    pub frame_rate: usize,
    pub frames: usize,
    pub core_count: usize,
    pub speed: f32,
    pub frame_dir: String,
    pub output: String,
    pub output_format: OutputFormat,
    pub shader: shader::ShaderFn,
    pub shader_name: String,
}

impl Config {
    pub fn parse_config_file() -> Config {
        let cfg = std::fs::read_to_string("./config").expect("config file exists");
        let cfg: HashMap<&str, &str> = cfg
            .lines()
            .filter(|l| !l.starts_with("#") && !l.is_empty()) // ignore comments
            .map(|l| l.split_once("=").expect("value=key"))
            .map(|(k, v)| (k.trim(), v.trim()))
            .collect();

        let get_usize = |value: &str| {
            cfg.get(value)
                .expect(format!("{value} not given in config").as_str())
                .parse::<usize>()
                .expect(format!("invalid {value} value").as_str())
        };

        let width = get_usize("width");
        let height = get_usize("height");
        let frame_rate = get_usize("frame_rate");
        let frames = get_usize("frames");
        let core_count = get_usize("core_count");
        let speed = cfg
            .get("speed")
            .expect("speed not given in config")
            .parse::<f32>()
            .expect("invalid speed value");

        let frame_dir = cfg
            .get("frame_dir")
            .expect("frame_dir not given in config")
            .to_string();

        assert!(
            PathBuf::from(&frame_dir).is_dir(),
            "frame_dir must be a directory!"
        );

        let output = cfg
            .get("output")
            .expect("output not given in config")
            .to_string();

        let extension = Path::new(&output)
            .extension()
            .expect("must specify output extension")
            .to_str()
            .unwrap();

        let output_format = match extension {
            "mp4" => OutputFormat::MP4,
            "gif" => OutputFormat::GIF,
            _ => panic!("invalid output extension! Only support mp4 or gif!"),
        };

        let shader_name = cfg
            .get("shader")
            .expect("shader not gien in config")
            .to_string();
        let shader = shader::get_shader(&shader_name).expect("invalid shader");

        Config {
            width,
            height,
            frame_rate,
            frames,
            core_count,
            speed,
            frame_dir,
            output,
            output_format,
            shader,
            shader_name,
        }
    }
}
