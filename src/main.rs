use std::{
    fs::{self, File},
    io::{self, Write, stdout},
    path::Path,
    process::Command,
};

mod math;
mod shader;

use shader::ShaderArgs;

use crate::math::Vec2;

const WIDTH: usize = 720;
const HEIGHT: usize = 480;
const COLOR_DEPTH: usize = 255;
const FRAME_COUNT: usize = 60;

const FRAME_DIR: &str = "./frames";
const OUTPUT_VIDEO: &str = "render.mp4";

fn main() {
    setup();

    let header = format!("P6\n{WIDTH}\n{HEIGHT}\n{COLOR_DEPTH}\n").into_bytes();

    let mut shader_args = ShaderArgs {
        frag_coord: Vec2::zero(),
        resolution: Vec2::new(WIDTH as f32, HEIGHT as f32),
        time: 0.0,
    };

    for i in 0..FRAME_COUNT {
        let mut data = [0u8; WIDTH * HEIGHT * 3];
        shader_args.time = i as f32 / FRAME_COUNT as f32;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                shader_args.frag_coord.x = x as f32;
                shader_args.frag_coord.y = y as f32;

                let frag = shader::cyberspace(&shader_args);
                let i = (y * WIDTH + x) * 3;
                data[i] = (frag.x * 255.0) as u8;
                data[i + 1] = (frag.y * 255.0) as u8;
                data[i + 2] = (frag.z * 255.0) as u8;
            }
        }

        let file_path = format!("{FRAME_DIR}/f{i}.ppm");
        let mut file = File::create(&file_path).unwrap();
        file.write(&header).unwrap();
        file.write(&data).unwrap();

        print!("\rframe {}/{FRAME_COUNT}", i + 1);
        stdout().flush().unwrap();
    }

    println!();
    println!("rendering...");
    render(OUTPUT_VIDEO).unwrap();
    println!("finished! output: {OUTPUT_VIDEO}");
}

fn render(output: &str) -> io::Result<std::process::ExitStatus> {
    Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-y", // overwrite output.mp4 if it exists
            "-framerate",
            "30",
            "-i",
            &format!("{FRAME_DIR}/f%d.ppm"),
            "-pix_fmt",
            "yuv420p",
            output,
        ])
        .status()
}

fn setup() {
    let ffmpeg_exists = Command::new("ffmpeg").arg("-version").output().is_ok();
    if !ffmpeg_exists {
        panic!("ffmpeg not found in PATH. Please install ffmpeg.");
    }

    if Path::new(FRAME_DIR).exists() {
        fs::remove_dir_all(FRAME_DIR).unwrap();
        fs::create_dir(FRAME_DIR).unwrap();
    } else {
        fs::create_dir(FRAME_DIR).unwrap();
    }
}
