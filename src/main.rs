use std::{
    fs::{self},
    io::{Write, stdout},
    path::Path,
    process::{Command, Stdio},
    sync::mpsc::{self, Sender},
    thread,
    time::Instant,
};

mod config;
mod math;
mod shader;

use config::Config;
use shader::ShaderArgs;

fn main() {
    let cfg = Config::parse_config_file();
    setup(&cfg.frame_dir);
    let shader_args = ShaderArgs::new(cfg.width, cfg.height, cfg.frames, cfg.shader);
    let (tx, rx) = mpsc::channel::<()>();
    let time = Instant::now();

    for core_idx in 0..cfg.core_count {
        // divvy up the frames via round robin
        let frames = (core_idx..cfg.frames).step_by(cfg.core_count).collect();
        let args = shader_args.clone();
        let tx1 = tx.clone();
        let frame_dir = cfg.frame_dir.clone();
        thread::spawn(|| render(args, frames, tx1, frame_dir));
    }
    drop(tx);

    println!("Rendering using {} cores...", cfg.core_count);
    print!("\rframe 0/{}", cfg.frames);
    stdout().flush().unwrap();

    let mut frames_rendered = 0;
    for _frame_finisehd in rx {
        frames_rendered += 1;
        print!("\rframe {frames_rendered}/{}", cfg.frames);
        stdout().flush().unwrap();
    }

    let elapsed = time.elapsed();
    println!("\nrendered {} frames in {elapsed:.2?}", cfg.frames);

    make_video(&cfg.frame_dir, &cfg.output, cfg.frame_rate);
}

fn render(mut shader_args: ShaderArgs, frames: Vec<usize>, tx: Sender<()>, frame_dir: String) {
    let width = shader_args.resolution.x as usize;
    let height = shader_args.resolution.y as usize;
    let mut pixels = vec![0u8; width * height * 3];

    for f in frames {
        shader_args.time = f as f32 / shader_args.frame_count;
        for y in 0..height {
            for x in 0..width {
                shader_args.frag_coord.x = x as f32;
                shader_args.frag_coord.y = y as f32;

                let frag = (shader_args.shader)(&shader_args);
                let i = (y * width + x) * 3;
                pixels[i] = (frag.x * 255.0) as u8;
                pixels[i + 1] = (frag.y * 255.0) as u8;
                pixels[i + 2] = (frag.z * 255.0) as u8;
            }
        }
        save_frame(&pixels, f, width, height, &frame_dir);
        tx.send(()).unwrap();
    }
}

fn save_frame(frame: &[u8], idx: usize, width: usize, height: usize, frame_dir: &str) {
    let output = format!("{frame_dir}/f{idx}.png");

    let mut ffmpeg = Command::new("ffmpeg")
        .args([
            "-y", // overwrite output
            "-f",
            "rawvideo",
            "-pixel_format",
            "rgb24",
            "-video_size",
            &format!("{}x{}", width, height),
            "-i",
            "-", // read from stdin
            "-frames:v",
            "1",
            &output,
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("failed to spawn ffmpeg");

    ffmpeg
        .stdin
        .as_mut()
        .expect("failed to open stdin")
        .write_all(frame)
        .expect("failed to write frame");

    let status = ffmpeg.wait().expect("ffmpeg failed");
    assert!(status.success());
}

fn make_video(frame_dir: &str, output: &str, frame_rate: usize) {
    println!("Making video...");
    Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-y", // overwrite output.mp4 if it exists
            "-framerate",
            &frame_rate.to_string(),
            "-i",
            &format!("{frame_dir}/f%d.png"),
            "-pix_fmt",
            "yuv420p",
            output,
        ])
        .status()
        .unwrap();
    println!("finished! output: {output}");
}

fn setup(frame_dir: &str) {
    let ffmpeg_exists = Command::new("ffmpeg").arg("-version").output().is_ok();
    if !ffmpeg_exists {
        panic!("ffmpeg not found in PATH. Please install ffmpeg.");
    }

    if Path::new(frame_dir).exists() {
        fs::remove_dir_all(frame_dir).unwrap();
        fs::create_dir(frame_dir).unwrap();
    } else {
        fs::create_dir(frame_dir).unwrap();
    }
}
