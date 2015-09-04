extern crate num;
extern crate glium;
extern crate rustc_serialize;

macro_rules! impl_uniform_block_basic {
    ($ty:ty, $uniform_ty:expr) => (
        impl ::glium::uniforms::UniformBlock for $ty {

            fn matches(layout: &::glium::program::BlockLayout, base_offset: usize)
                       -> Result<(), ::glium::uniforms::LayoutMismatchError>
            {
                if let &::glium::program::BlockLayout::BasicType { ty, offset_in_buffer } = layout {
                    if ty != $uniform_ty {
                        return Err(::glium::uniforms::LayoutMismatchError::TypeMismatch {
                            expected: ty,
                            obtained: $uniform_ty,
                        });
                    }

                    if offset_in_buffer != base_offset {
                        return Err(::glium::uniforms::LayoutMismatchError::OffsetMismatch {
                            expected: offset_in_buffer,
                            obtained: base_offset,
                        });
                    }

                    Ok(())

                } else {
                    Err(::glium::uniforms::LayoutMismatchError::LayoutMismatch {
                        expected: layout.clone(),
                        obtained: ::glium::program::BlockLayout::BasicType {
                            ty: $uniform_ty,
                            offset_in_buffer: base_offset,
                        }
                    })
                }
            }

            #[inline]
            fn build_layout(base_offset: usize) -> ::glium::program::BlockLayout {
                ::glium::program::BlockLayout::BasicType {
                    ty: $uniform_ty,
                    offset_in_buffer: base_offset,
                }
            }
        }
    )
}

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

