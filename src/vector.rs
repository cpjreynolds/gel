use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
};

use num::{
    Zero,
};

macro_rules! vec_self_binop_impl {
    ($imp:ident, $method:ident for $vec:ident, $($field:ident),+) => {
        impl $imp for $vec {
            type Output = Self;

            #[inline]
            fn $method(self, other: Self) -> Self::Output {
                $vec { $($field: self.$field.$method(other.$field)),+ }
            }
        }
        binop_ref_impl! { $imp, $method for $vec, $vec }
    }
}

macro_rules! vec_float_binop_impl {
    ($imp:ident, $method:ident for $vec:ident, $($field:ident),+) => {
        impl $imp<f32> for $vec {
            type Output = Self;

            #[inline]
            fn $method(self, other: f32) -> Self::Output {
                $vec { $($field: self.$field.$method(other)),+ }
            }
        }
        binop_ref_impl! { $imp, $method for $vec, f32 }
    }
}

macro_rules! vec_zero_impl {
    ($vec:ident, $($field:ident),+) => {
        impl Zero for $vec {
            fn zero() -> Self {
                $vec { $($field: f32::zero()),+ }
            }

            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }
    }
}

macro_rules! vec_new_impl {
    ($vec:ident, $($field:ident),+) => {
        impl $vec {
            fn new($($field: f32),+) -> $vec {
                $vec { $($field: $field),+ }
            }
        }
    }
}

macro_rules! vec_impl {
    ($vec:ident, $($field:ident),+) => {
        vec_new_impl!{ $vec, $($field),+ }
        vec_zero_impl!{ $vec, $($field),+ }

        vec_self_binop_impl!{ Add, add for $vec, $($field),+ }
        vec_self_binop_impl!{ Sub, sub for $vec, $($field),+ }

        vec_float_binop_impl!{ Add, add for $vec, $($field),+ }
        vec_float_binop_impl!{ Sub, sub for $vec, $($field),+ }
        vec_float_binop_impl!{ Mul, mul for $vec, $($field),+ }
        vec_float_binop_impl!{ Div, div for $vec, $($field),+ }
    }
}


#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
vec_impl!(Vec2, x, y);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
vec_impl!(Vec3, x, y, z);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
vec_impl!(Vec4, x, y, z, w);
