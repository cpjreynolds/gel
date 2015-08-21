use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    Deref,
    DerefMut,
};
use std::mem;

use num::{
    Zero,
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
    fn dot(&self, other: Self) -> f32;
}

pub trait Cross {
    fn cross(&self, other: Self) -> Self;
}

pub trait Normalize {
    fn normalize(&self) -> Self;
}

pub trait Length {
    fn length(&self) -> f32;
}

impl<T> Length for T
    where T: Dot + Copy
{
    fn length(&self) -> f32 {
        (self.dot(*self)).sqrt()
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

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vector for Vec2 {
    type Buffer = [f32; 2];
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] + other[i];
        }
        result
    }
}

impl Add<f32> for Vec2 {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] + other;
        }
        result
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] - other[i];
        }
        result
    }
}

impl Sub<f32> for Vec2 {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] - other;
        }
        result
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] * other;
        }
        result
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
         let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] + other;
        }
        result
    }
}

impl Dot for Vec2 {
    fn dot(&self, other: Self) -> f32 {
        let mut result = f32::zero();
        for i in 0..self.len() {
            result += self[i] * other[i];
        }
        result
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

impl AsRef<[f32; 2]> for Vec2 {
    fn as_ref<'a>(&'a self) -> &'a [f32; 2] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(ary: [f32; 2]) -> Self {
        unsafe {
            mem::transmute(ary)
        }
    }
}

impl Deref for Vec2 {
    type Target = <Self as Vector>::Buffer;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl DerefMut for Vec2 {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
        unsafe {
            mem::transmute(self)
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector for Vec3 {
    type Buffer = [f32; 3];
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] + other[i];
        }
        result
    }
}

impl Add<f32> for Vec3 {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] + other;
        }
        result
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] - other[i];
        }
        result
    }
}

impl Sub<f32> for Vec3 {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] - other;
        }
        result
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] * other;
        }
        result
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
         let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] + other;
        }
        result
    }
}

impl Dot for Vec3 {
    fn dot(&self, other: Self) -> f32 {
        let mut result = f32::zero();
        for i in 0..self.len() {
            result += self[i] * other[i];
        }
        result
    }
}

impl Cross for Vec3 {
    fn cross(&self, other: Self) -> Self {
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

impl AsRef<[f32; 3]> for Vec3 {
    fn as_ref<'a>(&'a self) -> &'a [f32; 3] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(ary: [f32; 3]) -> Self {
        unsafe {
            mem::transmute(ary)
        }
    }
}

impl Deref for Vec3 {
    type Target = <Self as Vector>::Buffer;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl DerefMut for Vec3 {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
        unsafe {
            mem::transmute(self)
        }
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

impl Vector for Vec4 {
    type Buffer = [f32; 4];
}

impl Add for Vec4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] + other[i];
        }
        result
    }
}

impl Add<f32> for Vec4 {
    type Output = Self;

    fn add(self, other: f32) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] + other;
        }
        result
    }
}

impl Sub for Vec4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] - other[i];
        }
        result
    }
}

impl Sub<f32> for Vec4 {
    type Output = Self;

    fn sub(self, other: f32) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] - other;
        }
        result
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] * other;
        }
        result
    }
}

impl Div<f32> for Vec4 {
    type Output = Self;

    fn div(self, other: f32) -> Self {
         let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] + other;
        }
        result
    }
}

impl Dot for Vec4 {
    fn dot(&self, other: Self) -> f32 {
        let mut result = f32::zero();
        for i in 0..self.len() {
            result += self[i] * other[i];
        }
        result
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

impl AsRef<[f32; 4]> for Vec4 {
    fn as_ref<'a>(&'a self) -> &'a [f32; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl From<[f32; 4]> for Vec4 {
    fn from(ary: [f32; 4]) -> Self {
        unsafe {
            mem::transmute(ary)
        }
    }
}

impl Deref for Vec4 {
    type Target = <Self as Vector>::Buffer;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl DerefMut for Vec4 {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
        unsafe {
            mem::transmute(self)
        }
    }
}

