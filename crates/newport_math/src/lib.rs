#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_float_classify)]
#![cfg_attr(target_arch = "spirv", feature(asm, register_attr, repr_simd))]
#![no_std]

#[cfg(target_arch = "spirv")]
mod spirv;

extern crate num_traits;
pub use num_traits::*;

pub const PI  : f32 = 3.141592;
pub const TAU : f32 = PI * 2.0;

pub const TO_RAD : f32 = PI / 180.0;
pub const TO_DEG : f32 = 180.0 / PI;

pub mod vec2;
pub use vec2::Vector2;

pub mod vec3;
pub use vec3::Vector3;

pub mod vec4;
pub use vec4::Vector4;

pub mod mat4;
pub use mat4::Matrix4;

pub mod color;
pub use color::Color;

pub mod rect;
pub use rect::*;

