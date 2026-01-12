use std::{
    fs::{self},
    io::{self, Write, stdout},
    path::Path,
    process::{Command, Stdio},
    sync::mpsc::{self, Sender},
    thread,
    time::Instant,
};

mod math;
mod shader;

use shader::ShaderArgs;

const WIDTH: usize = 720;
const HEIGHT: usize = 480;
const FRAME_COUNT: usize = 60;
const CORE_COUNT: usize = 4;

const FRAME_DIR: &str = "./frames";
const OUTPUT_VIDEO: &str = "render.mp4";

fn main() {
    setup();

    let shader_args = ShaderArgs::new(WIDTH, HEIGHT);
    let (tx, rx) = mpsc::channel::<()>();
    let time = Instant::now();

    for core_idx in 0..CORE_COUNT {
        // divvy up the frames via round robin
        let frames = (core_idx..FRAME_COUNT).step_by(CORE_COUNT).collect();
        let args = shader_args.clone();
        let tx1 = tx.clone();
        thread::spawn(|| render(args, frames, tx1));
    }
    drop(tx);

    println!("Rendering using {CORE_COUNT} cores...");
    print!("\rframe 0/{FRAME_COUNT}");
    stdout().flush().unwrap();

    let mut frames_rendered = 0;
    for _frame_finisehd in rx {
        frames_rendered += 1;
        print!("\rframe {frames_rendered}/{FRAME_COUNT}");
        stdout().flush().unwrap();
    }

    let elapsed = time.elapsed();
    println!("\nrendered {FRAME_COUNT} frames in {elapsed:.2?}");

    make_video(OUTPUT_VIDEO)
}

fn render(mut shader_args: ShaderArgs, frames: Vec<usize>, tx: Sender<()>) {
    let mut pixels = [0u8; WIDTH * HEIGHT * 3];
    for f in frames {
        shader_args.time = f as f32 / FRAME_COUNT as f32;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                shader_args.frag_coord.x = x as f32;
                shader_args.frag_coord.y = y as f32;

                let frag = (shader_args.shader)(&shader_args);
                let i = (y * WIDTH + x) * 3;
                pixels[i] = (frag.x * 255.0) as u8;
                pixels[i + 1] = (frag.y * 255.0) as u8;
                pixels[i + 2] = (frag.z * 255.0) as u8;
            }
        }
        save_frame(&pixels, f);
        tx.send(()).unwrap();
    }
}

fn save_frame(frame: &[u8], idx: usize) {
    let output = format!("{FRAME_DIR}/f{idx}.png");

    let mut ffmpeg = Command::new("ffmpeg")
        .args([
            "-y", // overwrite output
            "-f",
            "rawvideo",
            "-pixel_format",
            "rgb24",
            "-video_size",
            &format!("{}x{}", WIDTH, HEIGHT),
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

fn make_video(output: &str) {
    println!("Making video...");
    Command::new("ffmpeg")
        .args([
            "-hide_banner",
            "-loglevel",
            "error",
            "-y", // overwrite output.mp4 if it exists
            "-framerate",
            "30",
            "-i",
            &format!("{FRAME_DIR}/f%d.png"),
            "-pix_fmt",
            "yuv420p",
            output,
        ])
        .status()
        .unwrap();
    println!("finished! output: {OUTPUT_VIDEO}");
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
