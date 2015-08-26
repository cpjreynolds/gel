use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Neg,
    Deref,
    DerefMut,
};
use std::mem;

use num::{
    Zero,
};

use glium::uniforms::{
    AsUniformValue,
    UniformValue,
};

pub trait Vector: Sized + Copy
    + Dot + Normalize + Length
    + Add<Output=Self> + Add<f32, Output=Self>
    + Sub<Output=Self> + Sub<f32, Output=Self>
    + Div<f32, Output=Self> + Mul<f32, Output=Self>
{
    type Buffer;
}

pub trait Dot {
    fn dot(&self, other: &Self) -> f32;
}

pub trait Cross {
    fn cross(&self, other: &Self) -> Self;
}

pub trait Normalize {
    fn normalize(&self) -> Self;
}

pub trait Length {
    fn length(&self) -> f32;
}

pub trait Repeat {
    fn repeat(n: f32) -> Self;
}

impl<T> Length for T
    where T: Dot + Copy
{
    fn length(&self) -> f32 {
        self.dot(self).sqrt()
    }
}

impl<T> Normalize for T
    where T: Length + Mul<f32, Output=T> + Copy
{
    fn normalize(&self) -> Self {
        let scale = 1.0 / self.length();
        *self * scale
    }
}

macro_rules! vec_binop_vec_impl {
    ($imp:ident, $method:ident, $t:ty) => {
        impl $imp for $t {
            type Output = Self;

            fn $method(self, other: Self) -> Self::Output {
                let mut result = Self::zero();
                for i in 0..self.len() {
                    result[i] = self[i].$method(other[i]);
                }
                result
            }
        }
    }
}

macro_rules! vec_binop_scalar_impl {
    ($imp:ident, $method:ident, $t:ty) => {
        impl $imp<f32> for $t {
            type Output = Self;

            fn $method(self, other: f32) -> Self::Output {
                let mut result = Self::zero();
                for i in 0..self.len() {
                    result[i] = self[i].$method(other)
                }
                result
            }
        }
    }
}

macro_rules! vec_neg_impl {
    ($t:ty) => {
        impl Neg for $t {
            type Output = Self;

            fn neg(self) -> Self::Output {
                let mut result = Self::zero();
                for i in 0..self.len() {
                    result[i] = -self[i];
                }
                result
            }
        }
    }
}

macro_rules! vec_dot_impl {
    ($t:ty) => {
        impl Dot for $t {
            fn dot(&self, other: &Self) -> f32 {
                let mut result = f32::zero();
                for i in 0..self.len() {
                    result += self[i] * other[i];
                }
                result
            }
        }
    }
}

macro_rules! vec_as_ref_mut_impl {
    ($t:ty, $r:ty) => {
        impl AsRef<$r> for $t {
            fn as_ref(&self) -> &$r {
                unsafe {
                    mem::transmute(self)
                }
            }
        }

        impl AsMut<$r> for $t {
            fn as_mut(&mut self) -> &mut $r {
                unsafe {
                    mem::transmute(self)
                }
            }
        }
    }
}

macro_rules! vec_from_into_impl {
    ($t:ty, $r:ty) => {
        impl From<$r> for $t {
            fn from(ary: $r) -> Self {
                unsafe {
                    mem::transmute(ary)
                }
            }
        }

        impl Into<$r> for $t {
            fn into(self) -> $r {
                unsafe {
                    mem::transmute(self)
                }
            }
        }
    }
}

macro_rules! vec_deref_impl {
    ($t:ty, $r:ty) => {
        impl Deref for $t {
            type Target = $r;

            fn deref(&self) -> &Self::Target {
                unsafe {
                    mem::transmute(self)
                }
            }
        }

        impl DerefMut for $t {
            fn deref_mut(&mut self) -> &mut Self::Target {
                unsafe {
                    mem::transmute(self)
                }
            }
        }
    }
}

macro_rules! vec_impl {
    ($t:ty, $buf:ty) => {
        vec_binop_vec_impl!(Add, add, $t);
        vec_binop_vec_impl!(Sub, sub, $t);
        vec_binop_vec_impl!(Mul, mul, $t);
        vec_binop_vec_impl!(Div, div, $t);

        vec_binop_scalar_impl!(Add, add, $t);
        vec_binop_scalar_impl!(Sub, sub, $t);
        vec_binop_scalar_impl!(Mul, mul, $t);
        vec_binop_scalar_impl!(Div, div, $t);

        vec_neg_impl!($t);

        vec_dot_impl!($t);

        vec_as_ref_mut_impl!($t, $buf);
        vec_from_into_impl!($t, $buf);
        vec_deref_impl!($t, $buf);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
vec_impl!(Vec2, [f32; 2]);

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 {
            x: x,
            y: y,
        }
    }

    pub fn extend(&self, z: f32) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }
}

impl Vector for Vec2 {
    type Buffer = [f32; 2];
}

impl Repeat for Vec2 {
    fn repeat(n: f32) -> Self {
        Vec2 { x: n, y: n }
    }
}

impl Zero for Vec2 {
    fn zero() -> Self {
        Self::from([f32::zero(); 2])
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl AsUniformValue for Vec2 {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Vec2(unsafe {
            mem::transmute(*self)
        })
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
vec_impl!(Vec3, [f32; 3]);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }

    pub fn truncate(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn extend(&self, w: f32) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
    }
}

impl Vector for Vec3 {
    type Buffer = [f32; 3];
}

impl Repeat for Vec3 {
    fn repeat(n: f32) -> Self {
        Vec3 { x: n, y: n, z: n }
    }
}

impl Cross for Vec3 {
    fn cross(&self, other: &Self) -> Self {
        let mut result = Vec3::zero();
        result[0] = self[1] * other[2] - self[2] * other[1];
        result[1] = self[2] * other[0] - self[0] * other[2];
        result[2] = self[0] * other[1] - self[1] * other[0];
        result
    }
}

impl Zero for Vec3 {
    fn zero() -> Self {
        Self::from([f32::zero(); 3])
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl AsUniformValue for Vec3 {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Vec3(unsafe {
            mem::transmute(*self)
        })
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
vec_impl!(Vec4, [f32; 4]);

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        Vec4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }

    pub fn truncate(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl Vector for Vec4 {
    type Buffer = [f32; 4];
}

impl Repeat for Vec4 {
    fn repeat(n: f32) -> Self {
        Vec4 { x: n, y: n, z: n, w: n }
    }
}

impl Zero for Vec4 {
    fn zero() -> Self {
        Self::from([f32::zero(); 4])
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl AsUniformValue for Vec4 {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Vec4(unsafe {
            mem::transmute(*self)
        })
    }
}
