use std::mem;
use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
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
    Cross,
    Dot,
    Normalize,
};

pub trait Translate {
    fn translation(v: Vec3) -> Self;
    fn translate(&self, v: Vec3) -> Self;
}

pub trait Rotate {
    fn rotation(angle: f32, axis: Vec3) -> Self;
    fn rotate(&self, angle: f32, axis: Vec3) -> Self;
}

pub trait Inverse {
    fn inverse(&self) -> Self;
}

pub trait Transpose {
    fn transpose(&self) -> Self;
}

pub trait Scale {
    fn scaling(v: Vec3) -> Self;
    fn scale(&self, v: Vec3) -> Self;
}

pub trait LookAt {
    fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self;
}

pub trait Projection {
    fn perspective(fovy: f32, aspect: f32, znear: f32, zfar: f32) -> Self;
}

pub trait Matrix {
    type Buffer;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Mat4(pub [Vec4; 4]);

impl Mat4 {
    /// Create a new 4x4 matrix.
    pub fn new(m11: f32, m12: f32, m13: f32, m14: f32,
               m21: f32, m22: f32, m23: f32, m24: f32,
               m31: f32, m32: f32, m33: f32, m34: f32,
               m41: f32, m42: f32, m43: f32, m44: f32,) -> Mat4
    {
        Mat4([
            Vec4 { x: m11, y: m21, z: m31, w: m41, },
            Vec4 { x: m12, y: m22, z: m32, w: m42, },
            Vec4 { x: m13, y: m23, z: m33, w: m43, },
            Vec4 { x: m14, y: m24, z: m34, w: m44, },
        ])
    }
}

impl Matrix for Mat4 {
    type Buffer = [Vec4; 4];
}

impl Add for Mat4 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] + other[i];
        }
        result
    }
}

impl Sub for Mat4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] - other[i];
        }
        result
    }
}

impl Mul for Mat4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let mut result = Self::zero();

        for i in 0..self.len() {
            for j in 0..self.len() {
                let mut acc = f32::zero();

                for k in 0..self.len() {
                    acc += self[i][k] * other[k][j];
                }
                result[i][j] = acc;
            }
        }
        result
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        let mut result = Vec4::zero();
        for i in 0..self.len() {
            for j in 0..self.len() {
                result[i] += self[i][j] * other[j];
            }
        }
        result
    }
}

impl Add<f32> for Mat4 {
    type Output = Self;

    fn add(self, other: f32) -> Self::Output {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] + other;
        }
        result
    }
}

impl Sub<f32> for Mat4 {
    type Output = Self;

    fn sub(self, other: f32) -> Self::Output {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] - other;
        }
        result
    }
}

impl Mul<f32> for Mat4 {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] * other;
        }
        result
    }
}

impl Div<f32> for Mat4 {
    type Output = Self;

    fn div(self, other: f32) -> Self::Output {
        let mut result = Self::zero();
        for i in 0..self.len() {
            result[i] = self[i] / other;
        }
        result
    }
}

impl Inverse for Mat4 {
    fn inverse(&self) -> Self {
        unimplemented!()
    }
}

impl Zero for Mat4 {
    fn zero() -> Self {
        Self::from([Vec4::zero(); 4])
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl One for Mat4 {
    fn one() -> Self {
        let mut result = Self::zero();
        for i in 0..4 {
            result[i][i] = f32::one();
        }
        result
    }
}

impl Transpose for Mat4 {
    fn transpose(&self) -> Self {
        let mut result = Self::zero();
        for i in 0..self.len() {
            for j in 0..self.len() {
                result[j][i] = self[i][j];
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

    fn translate(&self, v: Vec3) -> Self {
        let mut result = self.clone();
        result[3] = self[0] * v[0] + self[1] * v[1] + self[2] * v[2] + self[3];
        result
    }
}

impl Rotate for Mat4 {
    fn rotation(angle: f32, axis: Vec3) -> Self {
        let a = angle;
        let c = a.cos();
        let s = a.sin();

        let axis = axis.normalize();

        let mut result = Self::one();

        result[0][0] = c + (1.0 - c) * axis.x * axis.x;
        result[0][1] = (1.0 - c) * axis.x * axis.y + s * axis.z;
        result[0][2] = (1.0 - c) * axis.x * axis.z - s * axis.y;
        result[0][3] = 0.0;

        result[1][0] = (1.0 - c) * axis.y * axis.x - s * axis.z;
        result[1][1] = c + (1.0 - c) * axis.y * axis.y;
        result[1][2] = (1.0 - c) * axis.y * axis.z + s * axis.x;
        result[1][3] = 0.0;

        result[2][0] = (1.0 - c) * axis.z * axis.x + s * axis.y;
        result[2][1] = (1.0 - c) * axis.z * axis.y - s * axis.x;
        result[2][2] = c + (1.0 - c) * axis.z * axis.z;
        result[2][3] = 0.0;

        result[3] = Vec4::new(0.0, 0.0, 0.0, 1.0);

        result
    }

    fn rotate(&self, angle: f32, axis: Vec3) -> Self {
        let r = Self::rotation(angle, axis);
        *self * r
    }
}

impl Projection for Mat4 {
    fn perspective(fovy: f32, aspect: f32, znear: f32, zfar: f32) -> Self {
        let tan_half_fovy = (fovy / 2.0).tan();
        let mut result = Self::zero();

        result[0][0] = 1.0 / (aspect * tan_half_fovy);
        result[1][1] = 1.0 / (tan_half_fovy);
        result[2][2] = (zfar + znear) / (zfar - znear);
        result[2][3] = 1.0;
        result[3][2] = -(2.0 * zfar * znear) / (zfar - znear);

        result
    }
}

impl Scale for Mat4 {
    fn scaling(v: Vec3) -> Self {
        let mut result = Self::one();

        result[3][0] = v.x;
        result[3][1] = v.y;
        result[3][2] = v.z;

        result
    }

    fn scale(&self, v: Vec3) -> Self {
        let mut result = Self::one();

        result[0] = self[0] * v[0];
        result[1] = self[1] * v[1];
        result[2] = self[2] * v[2];
        result[3] = self[3];

        result
    }
}

impl LookAt for Mat4 {
    fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        let f = (center - eye).normalize();
        let s = f.cross(&up).normalize();
        let u = s.cross(&f);

        let mut result = Self::one();

        result[0][0] = s.x;
        result[1][0] = s.y;
        result[2][0] = s.z;
        result[0][1] = u.x;
        result[1][1] = u.y;
        result[2][1] = u.z;
        result[0][2] = -f.x;
        result[1][2] = -f.y;
        result[2][2] = -f.z;
        result[3][0] = -(s.dot(&eye));
        result[3][1] = -(u.dot(&eye));
        result[3][2] = f.dot(&eye);

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

impl From<[Vec4; 4]> for Mat4 {
    fn from(ary: [Vec4; 4]) -> Self {
        unsafe {
            mem::transmute(ary)
        }
    }
}

impl Into<[Vec4; 4]> for Mat4 {
    fn into(self) -> [Vec4; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl AsRef<[Vec4; 4]> for Mat4 {
    fn as_ref<'a>(&'a self) -> &'a [Vec4; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl AsMut<[Vec4; 4]> for Mat4 {
    fn as_mut<'a>(&'a mut self) -> &'a mut [Vec4; 4] {
        unsafe {
            mem::transmute(self)
        }
    }
}

impl Deref for Mat4 {
    type Target = <Self as Matrix>::Buffer;

    fn deref<'a>(&'a self) -> &'a Self::Target {
        &self.0
    }
}

impl DerefMut for Mat4 {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use num::{
        Zero,
        One,
    };

    use super::*;

    #[test]
    fn test_matrix_new() {
        let mat = Mat4::new(1.0, 0.0, 1.0, 0.0,
                            0.0, 1.0, 0.0, 1.0,
                            1.0, 0.0, 1.0, 0.0,
                            0.0, 1.0, 0.0, 1.0,);
        for i in 0..4 {
            for j in 0..4 {
                if (i + j) % 2 == 0 {
                    assert!(mat[i][j] == 1.0);
                } else {
                    assert!(mat[i][j] == 0.0);
                }
            }
        }
    }

    #[test]
    fn test_matrix_transpose() {
        let mat1 = Mat4::new(1.0, 0.0, 0.0, 0.0,
                             1.0, 1.0, 0.0, 0.0,
                             1.0, 1.0, 1.0, 0.0,
                             1.0, 1.0, 1.0, 1.0,);

        let mat2 = Mat4::new(1.0, 1.0, 1.0, 1.0,
                             0.0, 1.0, 1.0, 1.0,
                             0.0, 0.0, 1.0, 1.0,
                             0.0, 0.0, 0.0, 1.0,);

        assert!(mat1 == mat2.transpose());
    }

    #[test]
    fn test_matrix_zero() {
        let mat = Mat4::zero();
        for i in 0..4 {
            for j in 0..4 {
                assert!(mat[i][j] == 0.0);
            }
        }
    }

    #[test]
    fn test_matrix_one() {
        let mat = Mat4::one();

        for i in 0..4 {
            for j in 0..4 {
                if i == j {
                    assert!(mat[i][j] == 1.0);
                } else {
                    assert!(mat[i][j] == 0.0);
                }
            }
        }
    }
}
