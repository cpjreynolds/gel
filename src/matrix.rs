use std::mem;
use std::ops::{
    Add,
    Sub,
    Mul,
    Deref,
    DerefMut,
};

use glium::uniforms::{
    AsUniformValue,
    UniformValue,
};

use num::{
    Zero,
    One,
};

use vector::{
    Vec3,
    Vec4,
};

pub trait Transpose {
    fn transpose(&self) -> Self;
}

pub trait Translate {
    fn translation(v: Vec3) -> Self;
}

pub trait Rotate {
    fn rotate(&self) -> Self;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Mat4(pub [Vec4; 4]);

impl Mat4 {
    pub fn get_row(&self, i: usize) -> Vec4 {
        let mut result = Vec4::zero();
        for j in 0..4 {
            result[j] = self[j][i];
        }
        result
    }

    pub fn get_col(&self, i: usize) -> Vec4 {
        let mut result = Vec4::zero();
        for j in 0..4 {
            result[j] = self[i][j];
        }
        result
    }
}

impl Add for Mat4 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = Mat4::zero();
        for i in 0..4 {
            result[i] = self[i] + other[i];
        }
        result
    }
}

impl Sub for Mat4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = Mat4::zero();
        for i in 0..4 {
            result[i] = self[i] - other[i];
        }
        result
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Mat4::zero();
        for col in 0..4 {
            for row in 0..4 {
                let mut sum = 0.0;
                for i in 0..4 {
                    sum += self[col][i] * other[i][row];
                }
                result[col][row] = sum;
            }
        }
        result
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        let mut result = Vec4::zero();
        for j in 0..4 {
            for i in 0..4 {
                result[j] += self[i][j] * other[i];
            }
        }
        result
    }
}

impl Mul<f32> for Mat4 {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        let mut result = Mat4::zero();
        for i in 0..4 {
            result[i] = self[i] * other;
        }
        result
    }
}

impl Zero for Mat4 {
    fn zero() -> Self {
        Mat4([Vec4::zero(); 4])
    }

    fn is_zero(&self) -> bool {
        *self == Mat4::zero()
    }
}

impl One for Mat4 {
    fn one() -> Self {
        let mut result = Mat4::zero();
        result[0][0] = 1.0;
        result[1][1] = 1.0;
        result[2][2] = 1.0;
        result[3][3] = 1.0;
        result
    }
}

impl AsRef<[Vec4; 4]> for Mat4 {
    fn as_ref<'a>(&'a self) -> &'a [Vec4; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl Deref for Mat4 {
    type Target = [Vec4; 4];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Mat4 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Transpose for Mat4 {
    fn transpose(&self) -> Self {
        let mut result = Mat4::zero();
        for j in 0..4 {
            for i in 0..4 {
                result[i][j] = self[j][i];
            }
        }
        result
    }
}

impl Translate for Mat4 {
    fn translation(v: Vec3) -> Self {
        let mut result = Mat4::one();
        result[3][0] = v[0];
        result[3][1] = v[1];
        result[3][2] = v[2];
        result
    }
}

impl AsUniformValue for Mat4 {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Mat4(unsafe {
            mem::transmute(*self)
        })
    }

}
