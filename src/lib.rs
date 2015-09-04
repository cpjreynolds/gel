extern crate num;
extern crate glium;
extern crate rustc_serialize;

mod vector;
mod matrix;


pub use vector::{
    Vec2,
    Vec3,
    Vec4,
};

pub use matrix::{
    Mat4,
};

pub use vector::{
    Vector,
    Dot,
    Cross,
    Normalize,
    Length,
    Repeat,
};

pub use matrix::{
    Translate,
    Rotate,
    Transpose,
    Inverse,
    Scale,
    LookAt,
    Projection,
    Matrix,
};

pub use num::{
    One,
    Zero,
};

pub use std::f32::consts::PI;
pub const PI_2: f32 = PI * 2.0;

pub fn radians(n: f32) -> f32 {
    n * (PI / 180.0)
}

pub fn degrees(n: f32) -> f32 {
    n * (180.0 / PI)
}
