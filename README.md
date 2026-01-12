# CPU Based Shader Renderer from Scratch

I made a multithreaded CPU based shader renderer in Rust. 🦀

![plasma](examples/plasma.gif)
![cyberspace](examples/cyberspace.gif)

_credit to @XorDev for the shaders_

## Features
 - zero dependencies except for ffmpeg (saving and stitching frames)
 - multithreaded 🦀
 - shader code is written in Rust with my custom math lib

## Try it Yourself

I recommend building it with max optimizations
 ```bash
 cargo build --release
 ```

 Copy `config.example` to `config`

 Configure the renderer & run it
