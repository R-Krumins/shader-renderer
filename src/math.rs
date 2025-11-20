use std::ops;
#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec3 {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { x: r, y: g, z: b }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }

    pub fn mix(a: Vec3, b: Vec3, t: f32) -> Self {
        Self {
            x: a.x * (1. - t) + b.x * t,
            y: a.y * (1. - t) + b.y * t,
            z: a.z * (1. - t) + b.z * t,
        }
    }
}

#[macro_export]
macro_rules! hex {
    ($hex:literal) => {{
        let hex_str: &str = $hex;
        let r = u8::from_str_radix(&hex_str[1..3], 16).unwrap() as f32 / 255.0;
        let g = u8::from_str_radix(&hex_str[3..5], 16).unwrap() as f32 / 255.0;
        let b = u8::from_str_radix(&hex_str[5..7], 16).unwrap() as f32 / 255.0;
        Vec3 { x: r, y: g, z: b }
    }};
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0., y: 0. }
    }
}

impl ops::Div<Vec2> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
