use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
};

use num::{
    Zero,
};

macro_rules! impl_vec_self_binop {
    (impl $imp:ident, $method:ident for $vec:ident { $($field:ident),+ }) => {
        impl $imp for $vec {
            type Output = Self;

            #[inline]
            fn $method(self, other: Self) -> Self::Output {
                $vec { $($field: self.$field.$method(other.$field)),+ }
            }
        }
        ref_binop! { impl $imp, $method for $vec, $vec }
    }
}

macro_rules! impl_vec_self_binops {
    (impl $vec:ident { $($field:ident),+ }) => {
        impl_vec_self_binop! { impl Add, add for $vec { $($field),+ } }
        impl_vec_self_binop! { impl Sub, sub for $vec { $($field),+ } }
    }
}



macro_rules! impl_vec_float_binop {
    (impl $imp:ident, $method:ident for $vec:ident { $($field:ident),+ }) => {
        impl $imp<f32> for $vec {
            type Output = Self;

            #[inline]
            fn $method(self, other: f32) -> Self::Output {
                $vec { $($field: self.$field.$method(other)),+ }
            }
        }
        ref_binop! { impl $imp, $method for $vec, f32 }
    }
}

macro_rules! impl_vec_float_binops {
    (impl $vec:ident { $($field:ident),+ }) => {
        impl_vec_float_binop! { impl Add, add for $vec { $($field),+ } }
        impl_vec_float_binop! { impl Sub, sub for $vec { $($field),+ } }
        impl_vec_float_binop! { impl Div, div for $vec { $($field),+ } }
        impl_vec_float_binop! { impl Mul, mul for $vec { $($field),+ } }
    }
}

macro_rules! impl_vec_zero {
    (impl $vec:ident { $($field:ident),+ }) => {
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

macro_rules! impl_vec {
    ($vec:ident { $($field:ident),+ }) => {

        impl $vec {
            fn new($($field: f32),+) -> $vec {
                $vec { $($field: $field),+ }
            }
        }

        impl_vec_self_binops! { impl $vec { $($field),+ } }
        impl_vec_float_binops! { impl $vec { $($field),+ } }
        impl_vec_zero! { impl $vec { $($field),+ } }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl_vec! { Vec2 { x, y } }

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl_vec! { Vec3 { x, y, z } }

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl_vec! { Vec4 { x, y, z, w } }
