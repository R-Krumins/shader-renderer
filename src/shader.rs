use crate::hex;
use crate::math::*;

pub struct ShaderArgs {
    pub frag_coord: Vec2,
    pub resolution: Vec2,
    pub time: f32,
}

pub fn gradient(args: &ShaderArgs) -> Vec3 {
    let c1 = hex!("#ff0000");
    let c2 = hex!("#00ff00");

    let uv = args.frag_coord / args.resolution;
    let mix_amount = uv.x * (1.0 - args.time);
    Vec3::mix(c1, c2, mix_amount)
}
