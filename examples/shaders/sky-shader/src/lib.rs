//! Ported to Rust from https://github.com/Tw1ddle/Sky-Shader/blob/master/src/shaders/glsl/sky.fragment

#![cfg_attr(target_arch = "spirv", no_std)]
#![feature(lang_items)]
#![feature(register_attr)]
#![register_attr(spirv)]

use core::f32::consts::PI;
use spirv_std::glam::{const_vec3, Mat4, Vec2, Vec3, Vec4};
use spirv_std::{Input, MathExt, Output};

const DEPOLARIZATION_FACTOR: f32 = 0.035;
const MIE_COEFFICIENT: f32 = 0.005;
const MIE_DIRECTIONAL_G: f32 = 0.8;
const MIE_K_COEFFICIENT: Vec3 = const_vec3!([0.686, 0.678, 0.666]);
const MIE_V: f32 = 4.0;
const MIE_ZENITH_LENGTH: f32 = 1.25e3;
const NUM_MOLECULES: f32 = 2.542e25f32;
const PRIMARIES: Vec3 = const_vec3!([6.8e-7f32, 5.5e-7f32, 4.5e-7f32]);
const RAYLEIGH: f32 = 1.0;
const RAYLEIGH_ZENITH_LENGTH: f32 = 8.4e3;
const REFRACTIVE_INDEX: f32 = 1.0003;
const SUN_ANGULAR_DIAMETER_DEGREES: f32 = 0.0093333;
const SUN_INTENSITY_FACTOR: f32 = 1000.0;
const SUN_INTENSITY_FALLOFF_STEEPNESS: f32 = 1.5;
const TURBIDITY: f32 = 2.0;

// TODO: add this to glam? Rust std has it on f32/f64
fn pow(v: Vec3, power: f32) -> Vec3 {
    Vec3::new(v.x().pow(power), v.y().pow(power), v.z().pow(power))
}

// TODO: add this to glam? Rust std has it on f32/f64
fn exp(v: Vec3) -> Vec3 {
    Vec3::new(v.x().exp(), v.y().exp(), v.z().exp())
}

/// Based on: https://seblagarde.wordpress.com/2014/12/01/inverse-trigonometric-functions-gpu-optimization-for-amd-gcn-architecture/
fn acos_approx(v: f32) -> f32 {
    let x = v.abs();
    let mut res = -0.155972 * x + 1.56467; // p(x)
    res *= (1.0f32 - x).sqrt();

    if v >= 0.0 {
        res
    } else {
        PI - res
    }
}

/// renamed because of cross-compilation issues with spirv-cross/ moltenvk
fn my_smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    // Scale, bias and saturate x to 0..1 range
    let x = ((x - edge0) / (edge1 - edge0)).saturate();
    // Evaluate polynomial
    x * x * (3.0 - 2.0 * x)
}

fn total_rayleigh(lambda: Vec3) -> Vec3 {
    (8.0 * PI.pow(3.0)
        * (REFRACTIVE_INDEX.pow(2.0) - 1.0).pow(2.0)
        * (6.0 + 3.0 * DEPOLARIZATION_FACTOR))
        / (3.0 * NUM_MOLECULES * pow(lambda, 4.0) * (6.0 - 7.0 * DEPOLARIZATION_FACTOR))
}

fn total_mie(lambda: Vec3, k: Vec3, t: f32) -> Vec3 {
    let c = 0.2 * t * 10e-18;
    0.434 * c * PI * pow((2.0 * PI) / lambda, MIE_V - 2.0) * k
}

fn rayleigh_phase(cos_theta: f32) -> f32 {
    (3.0 / (16.0 * PI)) * (1.0 + cos_theta.pow(2.0))
}

fn henyey_greenstein_phase(cos_theta: f32, g: f32) -> f32 {
    (1.0 / (4.0 * PI)) * ((1.0 - g.pow(2.0)) / (1.0 - 2.0 * g * cos_theta + g.pow(2.0)).pow(1.5))
}

fn sun_intensity(zenith_angle_cos: f32) -> f32 {
    let cutoff_angle = PI / 1.95; // Earth shadow hack
    SUN_INTENSITY_FACTOR
        * 0.0f32.max(
            1.0 - (-((cutoff_angle - acos_approx(zenith_angle_cos))
                / SUN_INTENSITY_FALLOFF_STEEPNESS))
                .exp(),
        )
}

fn tonemap(col: Vec3) -> Vec3 {
    // see https://www.desmos.com/calculator/0eo9pzo1at
    const A: f32 = 2.35;
    const B: f32 = 2.8826666;
    const C: f32 = 789.7459;
    const D: f32 = 0.935;

    let z = pow(col, A);
    z / (pow(z, D) * B + Vec3::splat(C))
}

fn sky(dir: Vec3, sun_position: Vec3) -> Vec3 {
    let up = Vec3::new(0.0, 1.0, 0.0);
    let sunfade = 1.0 - (1.0 - (sun_position.y() / 450000.0).exp()).saturate();
    let rayleigh_coefficient = RAYLEIGH - (1.0 * (1.0 - sunfade));
    let beta_r = total_rayleigh(PRIMARIES) * rayleigh_coefficient;

    // Mie coefficient
    let beta_m = total_mie(PRIMARIES, MIE_K_COEFFICIENT, TURBIDITY) * MIE_COEFFICIENT;

    // Optical length, cutoff angle at 90 to avoid singularity
    let zenith_angle = acos_approx(up.dot(dir).max(0.0));
    let denom = (zenith_angle).cos() + 0.15 * (93.885 - ((zenith_angle * 180.0) / PI)).pow(-1.253);

    let s_r = RAYLEIGH_ZENITH_LENGTH / denom;
    let s_m = MIE_ZENITH_LENGTH / denom;

    // Combined extinction factor
    let fex = exp(-(beta_r * s_r + beta_m * s_m));

    // In-scattering
    let sun_direction = sun_position.normalize();
    let cos_theta = dir.dot(sun_direction);
    let beta_r_theta = beta_r * rayleigh_phase(cos_theta * 0.5 + 0.5);

    let beta_m_theta = beta_m * henyey_greenstein_phase(cos_theta, MIE_DIRECTIONAL_G);
    let sun_e = sun_intensity(sun_direction.dot(up));
    let mut lin = pow(
        sun_e * ((beta_r_theta + beta_m_theta) / (beta_r + beta_m)) * (Vec3::splat(1.0) - fex),
        1.5,
    );

    lin *= Vec3::splat(1.0).lerp(
        pow(
            sun_e * ((beta_r_theta + beta_m_theta) / (beta_r + beta_m)) * fex,
            0.5,
        ),
        ((1.0 - up.dot(sun_direction)).pow(5.0)).saturate(),
    );

    // Composition + solar disc
    let sun_angular_diameter_cos = SUN_ANGULAR_DIAMETER_DEGREES.cos();
    let sundisk = my_smoothstep(
        sun_angular_diameter_cos,
        sun_angular_diameter_cos + 0.00002,
        cos_theta,
    );
    let mut l0 = 0.1 * fex;
    l0 += sun_e * 19000.0 * fex * sundisk;

    lin + l0
}

pub fn fs(screen_pos: Vec2) -> Vec4 {
    // hard-code information because we can't bind buffers at the moment
    let eye_pos = Vec3::new(0.0, 0.0997, 0.2);
    let sun_pos = Vec3::new(0.0, 75.0, -1000.0);
    let clip_to_world = Mat4::from_cols(
        Vec4::new(-0.5522849, 0.0, 0.0, 0.0),
        Vec4::new(0.0, 0.4096309, -0.061444636, 0.0),
        Vec4::new(0.0, 99.99999, 199.99998, 999.99994),
        Vec4::new(0.0, -0.14834046, -0.98893654, 0.0),
    );

    let world_pos = clip_to_world.transform_point3(screen_pos.extend(1.0));
    let dir = (world_pos - eye_pos).normalize();

    // evaluate Preetham sky model
    let color = sky(dir, sun_pos);

    // Tonemapping
    let color = color.max(Vec3::splat(0.0)).min(Vec3::splat(1024.0));

    tonemap(color).extend(1.0)
}

#[allow(unused_attributes)]
#[spirv(fragment)]
pub fn main_fs(in_pos: Input<Vec2>, mut output: Output<Vec4>) {
    let color = fs(in_pos.load());
    output.store(color);
}

#[allow(unused_attributes)]
#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_idx: Input<i32>,
    #[spirv(position)] mut builtin_pos: Output<Vec4>,
    mut out_pos: Output<Vec2>,
) {
    let vert_idx = vert_idx.load();

    // Create a "full screen triangle" by mapping the vertex index.
    // ported from https://www.saschawillems.de/blog/2016/08/13/vulkan-tutorial-on-rendering-a-fullscreen-quad-without-buffers/
    let uv = Vec2::new(((vert_idx << 1) & 2) as f32, (vert_idx & 2) as f32);
    let pos = 2.0 * uv - Vec2::one();

    builtin_pos.store(pos.extend(0.0).extend(1.0));
    out_pos.store(pos);
}

#[cfg(all(not(test), target_arch = "spirv"))]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(all(not(test), target_arch = "spirv"))]
#[lang = "eh_personality"]
extern "C" fn rust_eh_personality() {}
