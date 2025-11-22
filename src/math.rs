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

macro_rules! impl_vec3_swizzles {
    ($($name:ident: $a:ident $b:ident $c:ident),* $(,)?) => {
        $(
            pub fn $name(&self) -> Vec3 {
                Vec3::new(self.$a, self.$b, self.$c)
            }
        )*
    };
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

    impl_vec3_swizzles! {
        xyz: x y z,
        xzy: x z y,
        yxz: y x z,
        yzx: y z x,
        zxy: z x y,
        zyx: z y x,
        xyy: x y y,
        yxy: y x y,
    }

    pub fn mix(a: Vec3, b: Vec3, t: f32) -> Self {
        Self {
            x: a.x * (1. - t) + b.x * t,
            y: a.y * (1. - t) + b.y * t,
            z: a.z * (1. - t) + b.z * t,
        }
    }

    pub fn normalize(self) -> Self {
        let len = f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    pub fn sin(self) -> Self {
        Self {
            x: self.x.sin(),
            y: self.y.sin(),
            z: self.z.sin(),
        }
    }

    pub fn cos(self) -> Self {
        Self {
            x: self.x.cos(),
            y: self.y.cos(),
            z: self.z.cos(),
        }
    }

    pub fn tanh(self) -> Self {
        Self {
            x: self.x.tanh(),
            y: self.y.tanh(),
            z: self.z.tanh(),
        }
    }

    pub fn round(self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
        }
    }

    pub fn len(self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
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

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f32) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}
