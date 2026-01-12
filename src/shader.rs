use std::collections::HashMap;

use crate::hex;
use crate::math::*;

pub type ShaderFn = fn(&ShaderArgs) -> Vec3;

pub const SHADER_REGISTER: &[(&str, ShaderFn)] = &[("cyberspace", cyberspace), ("plasma", plasma)];

pub fn get_shader(name: &str) -> Option<ShaderFn> {
    SHADER_REGISTER
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, f)| *f)
}

#[derive(Clone)]
pub struct ShaderArgs {
    pub frag_coord: Vec2,
    pub resolution: Vec2,
    pub frame_count: f32,
    pub time: f32,
    pub speed: f32,
    pub shader: ShaderFn,
}

impl ShaderArgs {
    pub fn new(width: usize, height: usize, frame_count: usize, speed: f32, shader: ShaderFn) -> Self {
        Self {
            frag_coord: Vec2::zero(),
            resolution: Vec2::new(width as f32, height as f32),
            frame_count: frame_count as f32,
            time: 0.0,
            speed,
            shader,
        }
    }
}

#[allow(dead_code)]
pub fn gradient(args: &ShaderArgs) -> Vec3 {
    let c1 = hex!("#ff0000");
    let c2 = hex!("#00ff00");

    let uv = args.frag_coord / args.resolution;
    let mix_amount = uv.x * (1.0 - args.time);
    Vec3::mix(c1, c2, mix_amount)
}

pub fn cyberspace(args: &ShaderArgs) -> Vec3 {
    let mut i = 0.0;
    let mut z = 0.0;
    let mut d = 0.0;
    let mut o = Vec3::zero();

    let r = Vec3::new(args.resolution.x, args.resolution.y, args.resolution.y);
    let fc = Vec3::new(args.frag_coord.x, args.frag_coord.y, 1.0);
    let a = (fc * 2.0 - r).normalize();

    while z + i < 7e1 {
        i += 1.0;
        let mut p = a * z;
        p = p.abs();
        p.z += args.time * 5.0;
        p += Vec3::sin(p + p);
        d = 0.0;
        while d < 9.0 {
            d += 1.0;
            let b = Vec3::round(p * (0.2 * d));
            let b = Vec3::cos(b + 0.2 * args.time);
            let b = Vec3::new(b.z, b.x, b.y);
            p += b * 0.4;
        }
        d = 0.1 * f32::sqrt((p.xyy() * p.yxy()).len());
        z += d;
        o += Vec3::new(z, 1.0, 9.0) / d;
    }
    (o / 7e3).tanh()
}

pub fn plasma(args: &ShaderArgs) -> Vec3 {
    let p = (args.frag_coord * 2.0 - args.resolution) / args.resolution.y;
    let l = Vec2::zero() + (0.7 - p.dot(p)).abs();
    let mut v = p * Vec2::new(1.0 - l.x, 1.0 - l.y) / 0.2;
    let mut o = Vec3::zero();

    let mut i = 1.0;
    while i <= 8.0 {
        o += (v.xyy().sin() + 1.0) * (v.x - v.y).abs() * 0.2;
        v += (v.yx() * i + Vec2::new(0.0, i) + args.time).cos() / i + 0.7;
        i += 1.0;
    }
    let vertical = (Vec3::new(1.0, -1.0, -2.0) * p.y).exp();
    let atten = (-4.0 * l.x).exp();
    (vertical * atten / o).tanh()
}
