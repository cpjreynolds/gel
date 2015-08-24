extern crate num;
extern crate glium;

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
