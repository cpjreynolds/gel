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
    UniformType,
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
    Repeat,
    Normalize,
};

pub trait Translate {
    fn translation(v: Vec3) -> Self;
    fn translate(&self, v: Vec3) -> Self;
    fn translate_mut(&mut self, v: Vec3);
}

pub trait Rotate {
    fn rotation(angle: f32, axis: Vec3) -> Self;
    fn rotate(&self, angle: f32, axis: Vec3) -> Self;
    fn rotate_mut(&mut self, angle: f32, axis: Vec3);
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
    fn scale_mut(&mut self, v: Vec3);
}

pub trait LookAt {
    fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self;
}

pub trait Projection {
    fn perspective(fovy: f32, aspect: f32, znear: f32, zfar: f32) -> Self;
    fn project(obj: Vec3, model: Mat4, proj: Mat4, viewport: Vec4) -> Vec3;
    fn unproject(win: Vec3, model: Mat4, proj: Mat4, viewport: Vec4) -> Vec3;
}

pub trait Matrix {
    type Buffer;
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, RustcDecodable)]
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

    fn mul(self, right: Self) -> Self::Output {
        let srca0 = self[0];
        let srca1 = self[1];
        let srca2 = self[2];
        let srca3 = self[3];

        let srcb0 = right[0];
        let srcb1 = right[1];
        let srcb2 = right[2];
        let srcb3 = right[3];

        let mut result = Mat4::zero();
        result[0] = srca0 * srcb0[0] + srca1 * srcb0[1] + srca2 * srcb0[2] + srca3 * srcb0[3];
        result[1] = srca0 * srcb1[0] + srca1 * srcb1[1] + srca2 * srcb1[2] + srca3 * srcb1[3];
        result[2] = srca0 * srcb2[0] + srca1 * srcb2[1] + srca2 * srcb2[2] + srca3 * srcb2[3];
        result[3] = srca0 * srcb3[0] + srca1 * srcb3[1] + srca2 * srcb3[2] + srca3 * srcb3[3];

        result
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, right: Vec4) -> Vec4 {
        let mov0 = Vec4::repeat(right[0]);
        let mov1 = Vec4::repeat(right[1]);
        let mul0 = self[0] * mov0;
        let mul1 = self[1] * mov1;
        let add0 = mul0 + mul1;
        let mov2 = Vec4::repeat(right[2]);
        let mov3 = Vec4::repeat(right[3]);
        let mul2 = self[2] * mov2;
        let mul3 = self[3] * mov3;
        let add1 = mul2 + mul3;
        let add2 = add0 + add1;

        add2
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
        let coef00 = self[2][2] * self[3][3] - self[3][2] * self[2][3];
        let coef02 = self[1][2] * self[3][3] - self[3][2] * self[1][3];
        let coef03 = self[1][2] * self[2][3] - self[2][2] * self[1][3];

        let coef04 = self[2][1] * self[3][3] - self[3][1] * self[2][3];
        let coef06 = self[1][1] * self[3][3] - self[3][1] * self[1][3];
        let coef07 = self[1][1] * self[2][3] - self[2][1] * self[1][3];

        let coef08 = self[2][1] * self[3][2] - self[3][1] * self[2][2];
        let coef10 = self[1][1] * self[3][2] - self[3][1] * self[1][2];
        let coef11 = self[1][1] * self[2][2] - self[2][1] * self[1][2];

        let coef12 = self[2][0] * self[3][3] - self[3][0] * self[2][3];
        let coef14 = self[1][0] * self[3][3] - self[3][0] * self[1][3];
        let coef15 = self[1][0] * self[2][3] - self[2][0] * self[1][3];

        let coef16 = self[2][0] * self[3][2] - self[3][0] * self[2][2];
        let coef18 = self[1][0] * self[3][2] - self[3][0] * self[1][2];
        let coef19 = self[1][0] * self[2][2] - self[2][0] * self[1][2];

        let coef20 = self[2][0] * self[3][1] - self[3][0] * self[2][1];
        let coef22 = self[1][0] * self[3][1] - self[3][0] * self[1][1];
        let coef23 = self[1][0] * self[2][1] - self[2][0] * self[1][1];

        let fac0 = Vec4::new(coef00, coef00, coef02, coef03);
        let fac1 = Vec4::new(coef04, coef04, coef06, coef07);
        let fac2 = Vec4::new(coef08, coef08, coef10, coef11);
        let fac3 = Vec4::new(coef12, coef12, coef14, coef15);
        let fac4 = Vec4::new(coef16, coef16, coef18, coef19);
        let fac5 = Vec4::new(coef20, coef20, coef22, coef23);

        let vec0 = Vec4::new(self[1][0], self[0][0], self[0][0], self[0][0]);
        let vec1 = Vec4::new(self[1][1], self[0][1], self[0][1], self[0][1]);
        let vec2 = Vec4::new(self[1][2], self[0][2], self[0][2], self[0][2]);
        let vec3 = Vec4::new(self[1][3], self[0][3], self[0][3], self[0][3]);

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let sign_a = Vec4::new(1.0, -1.0, 1.0, -1.0);
        let sign_b = Vec4::new(-1.0, 1.0, -1.0, 1.0);
        let inverse = Mat4::from([inv0 * sign_a, inv1 * sign_b, inv2 * sign_a, inv3 * sign_b]);

        let row0 = Vec4::new(inverse[0][0], inverse[1][0], inverse[2][0], inverse[3][0]);

        let dot0 = self[0] * row0;

        let dot1 = (dot0.x + dot0.y) + (dot0.z + dot0.w);

        let one_over_determinant = 1.0 / dot1;

        inverse * one_over_determinant
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
        Mat4::one().translate(v)
    }

    fn translate(&self, v: Vec3) -> Self {
        let mut result = self.clone();
        result[3] = self[0] * v[0] + self[1] * v[1] + self[2] * v[2] + self[3];
        result
    }

    fn translate_mut(&mut self, v: Vec3) {
        *self = self.translate(v);
    }
}

impl Rotate for Mat4 {
    fn rotation(angle: f32, axis: Vec3) -> Self {
        Mat4::one().rotate(angle, axis)
    }

    fn rotate(&self, angle: f32, axis: Vec3) -> Self {
        let a = angle;
        let c = a.cos();
        let s = a.sin();

        let axis = axis.normalize();
        let temp = axis * (1.0 - c);

        let mut rotate = Self::zero();
        rotate[0][0] = c + temp[0] * axis[0];
        rotate[0][1] = 0.0 + temp[0] * axis[1] + s * axis[2];
        rotate[0][2] = 0.0 + temp[0] * axis[2] - s * axis[1];

        rotate[1][0] = 0.0 + temp[1] * axis[0] - s * axis[2];
        rotate[1][1] = c + temp[1] * axis[1];
        rotate[1][2] = 0.0 + temp[1] * axis[2] + s * axis[0];

        rotate[2][0] = 0.0 + temp[2] * axis[0] + s * axis[1];
        rotate[2][1] = 0.0 + temp[2] * axis[1] - s * axis[0];
        rotate[2][2] = c + temp[2] * axis[2];

        let mut result = Self::zero();
        result[0] = self[0] * rotate[0][0] + self[1] * rotate[0][1] + self[2] * rotate[0][2];
        result[1] = self[0] * rotate[1][0] + self[1] * rotate[1][1] + self[2] * rotate[1][2];
        result[2] = self[0] * rotate[2][0] + self[1] * rotate[2][1] + self[2] * rotate[2][2];
        result[3] = self[3];

        result
    }

    fn rotate_mut(&mut self, angle: f32, axis: Vec3) {
        *self = self.rotate(angle, axis);
    }
}

impl Projection for Mat4 {
    fn perspective(fovy: f32, aspect: f32, znear: f32, zfar: f32) -> Self {
        let tan_half_fovy = (fovy / 2.0).tan();

        let mut result = Self::zero();
        result[0][0] = 1.0 / (aspect * tan_half_fovy);
        result[1][1] = 1.0 / (tan_half_fovy);
        result[2][2] = -(zfar + znear) / (zfar - znear);
        result[2][3] = -1.0;
        result[3][2] = -(2.0 * zfar * znear) / (zfar - znear);

        result
    }

    fn project(obj: Vec3, modelview: Mat4, proj: Mat4, viewport: Vec4) -> Vec3 {
        let mut tmp = obj.extend(1.0);
        tmp = modelview * tmp;
        tmp = proj * tmp;

        tmp = tmp / tmp.w;
        tmp = tmp * 0.5 + 0.5;
        tmp[0] = tmp[0] * viewport[2] + viewport[0];
        tmp[1] = tmp[1] * viewport[3] + viewport[1];

        Vec3::new(tmp.x, tmp.y, tmp.z)
    }

    fn unproject(win: Vec3, modelview: Mat4, proj: Mat4, viewport: Vec4) -> Vec3 {
        let inverse = (proj * modelview).inverse();

        let mut tmp = win.extend(1.0);
        tmp.x = (tmp.x - viewport[0]) / viewport[2];
        tmp.y = (tmp.y - viewport[1]) / viewport[3];
        tmp = tmp * 2.0 - 1.0;

        let mut obj = inverse * tmp;
        obj = obj / obj.w;

        Vec3::new(obj.x, obj.y, obj.z)
    }
}

impl Scale for Mat4 {
    fn scaling(v: Vec3) -> Self {
        Mat4::one().scale(v)
    }

    fn scale(&self, v: Vec3) -> Self {
        let mut result = self.clone();
        result[0] = self[0] * v[0];
        result[1] = self[1] * v[1];
        result[2] = self[2] * v[2];
        result
    }

    fn scale_mut(&mut self, v: Vec3) {
        *self = self.scale(v);
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

impl_uniform_block_basic!(Mat4, UniformType::FloatMat4);

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

