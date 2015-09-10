//! `gel` is an extension of sebcrozet's `nalgebra` crate, designed to facilitate OpenGL
//! mathematics.

extern crate num;
extern crate nalgebra;
extern crate glium;
extern crate rustc_serialize;

use glium::uniforms::{
    AsUniformValue,
    UniformValue,
    UniformBlock,
};

pub use num::{
    One,
    Zero,
};

pub use nalgebra::*;
/// A trait for objects able to be extended.
pub trait Extend<N> {
    type Output;

    fn extend(&self, N) -> Self::Output;
}

/// A trait for objects able to be truncated.
pub trait Truncate {
    type Output;

    fn truncate(&self) -> Self::Output;
}

/// A trait for objects able to construct view matrices.
pub trait LookAt<N> {
    fn look_at(camera: &Vec3<N>, target: &Vec3<N>, up: &Vec3<N>) -> Self;
}

impl<N> Extend<N> for Vec0<N>
    where N: Copy
{
    type Output = Vec1<N>;

    fn extend(&self, elem: N) -> Self::Output {
        Vec1::new(elem)
    }
}

impl<N> Extend<N> for Vec1<N>
    where N: Copy
{
    type Output = Vec2<N>;

    fn extend(&self, elem: N) -> Self::Output {
        Vec2::new(self.x, elem)
    }
}

impl<N> Extend<N> for Vec2<N>
    where N: Copy
{
    type Output = Vec3<N>;

    fn extend(&self, elem: N) -> Self::Output {
        Vec3::new(self.x, self.y, elem)
    }
}

impl<N> Extend<N> for Vec3<N>
    where N: Copy
{
    type Output = Vec4<N>;

    fn extend(&self, elem: N) -> Self::Output {
        Vec4::new(self.x, self.y, self.z, elem)
    }
}

impl<N> Extend<N> for Vec4<N>
    where N: Copy
{
    type Output = Vec5<N>;

    fn extend(&self, elem: N) -> Self::Output {
        Vec5::new(self.x, self.y, self.z, self.w, elem)
    }
}

impl<N> Extend<N> for Vec5<N>
    where N: Copy
{
    type Output = Vec6<N>;

    fn extend(&self, elem: N) -> Self::Output {
        Vec6::new(self.x, self.y, self.z, self.w, self.a, elem)
    }
}

impl<N> Truncate for Vec1<N>
    where N: Copy
{
    type Output = Vec0<N>;

    fn truncate(&self) -> Self::Output {
        Vec0::new()
    }
}

impl<N> Truncate for Vec2<N>
    where N: Copy
{
    type Output = Vec1<N>;

    fn truncate(&self) -> Self::Output {
        Vec1::new(self.x)
    }
}

impl<N> Truncate for Vec3<N>
    where N: Copy
{
    type Output = Vec2<N>;

    fn truncate(&self) -> Self::Output {
        Vec2::new(self.x, self.y)
    }
}

impl<N> Truncate for Vec4<N>
    where N: Copy
{
    type Output = Vec3<N>;

    fn truncate(&self) -> Self::Output {
        Vec3::new(self.x, self.y, self.z)
    }
}

impl<N> Truncate for Vec5<N>
    where N: Copy
{
    type Output = Vec4<N>;

    fn truncate(&self) -> Self::Output {
        Vec4::new(self.x, self.y, self.z, self.w)
    }
}

impl<N> Truncate for Vec6<N>
    where N: Copy
{
    type Output = Vec5<N>;

    fn truncate(&self) -> Self::Output {
        Vec5::new(self.x, self.y, self.z, self.w, self.a)
    }
}

impl<N> Extend<N> for Pnt0<N>
    where N: Copy
{
    type Output = Pnt1<N>;

    fn extend(&self, elem: N) -> Self::Output {
        Pnt1::new(elem)
    }
}

impl<N> Extend<N> for Pnt1<N>
    where N: Copy
{
    type Output = Pnt2<N>;

    fn extend(&self, elem: N) -> Self::Output {
        Pnt2::new(self.x, elem)
    }
}

impl<N> Extend<N> for Pnt2<N>
    where N: Copy
{
    type Output = Pnt3<N>;

    fn extend(&self, elem: N) -> Self::Output {
        Pnt3::new(self.x, self.y, elem)
    }
}

impl<N> Extend<N> for Pnt3<N>
    where N: Copy
{
    type Output = Pnt4<N>;

    fn extend(&self, elem: N) -> Self::Output {
        Pnt4::new(self.x, self.y, self.z, elem)
    }
}

impl<N> Extend<N> for Pnt4<N>
    where N: Copy
{
    type Output = Pnt5<N>;

    fn extend(&self, elem: N) -> Self::Output {
        Pnt5::new(self.x, self.y, self.z, self.w, elem)
    }
}

impl<N> Extend<N> for Pnt5<N>
    where N: Copy
{
    type Output = Pnt6<N>;

    fn extend(&self, elem: N) -> Self::Output {
        Pnt6::new(self.x, self.y, self.z, self.w, self.a, elem)
    }
}

impl<N> Truncate for Pnt1<N>
    where N: Copy
{
    type Output = Pnt0<N>;

    fn truncate(&self) -> Self::Output {
        Pnt0::new()
    }
}

impl<N> Truncate for Pnt2<N>
    where N: Copy
{
    type Output = Pnt1<N>;

    fn truncate(&self) -> Self::Output {
        Pnt1::new(self.x)
    }
}

impl<N> Truncate for Pnt3<N>
    where N: Copy
{
    type Output = Pnt2<N>;

    fn truncate(&self) -> Self::Output {
        Pnt2::new(self.x, self.y)
    }
}

impl<N> Truncate for Pnt4<N>
    where N: Copy
{
    type Output = Pnt3<N>;

    fn truncate(&self) -> Self::Output {
        Pnt3::new(self.x, self.y, self.z)
    }
}

impl<N> Truncate for Pnt5<N>
    where N: Copy
{
    type Output = Pnt4<N>;

    fn truncate(&self) -> Self::Output {
        Pnt4::new(self.x, self.y, self.z, self.w)
    }
}

impl<N> Truncate for Pnt6<N>
    where N: Copy
{
    type Output = Pnt5<N>;

    fn truncate(&self) -> Self::Output {
        Pnt5::new(self.x, self.y, self.z, self.w, self.a)
    }
}

impl<N> LookAt<N> for Mat4<N>
    where N: BaseFloat
{
    fn look_at(camera: &Vec3<N>, target: &Vec3<N>, up: &Vec3<N>) -> Self {
        let f = (*target - *camera).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(&f);

        let mut result = Self::one();

        result.m11 = s.x;
        result.m12 = s.y;
        result.m13 = s.z;
        result.m21 = u.x;
        result.m22 = u.y;
        result.m23 = u.z;
        result.m31 = -f.x;
        result.m32 = -f.y;
        result.m33 = -f.z;
        result.m14 = -(s.dot(camera));
        result.m24 = -(u.dot(camera));
        result.m34 = f.dot(camera);

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub struct Perspective<N> {
    mat: Mat4<N>,
}

impl<N> Perspective<N>
    where N: BaseFloat
{
    pub fn new(fov: N, aspect: N, znear: N, zfar: N) -> Perspective<N> {
        let one = N::one();
        let two = N::one() + N::one();

        let tan_half_fov = (fov / two).tan();

        let mut mat = Mat4::zero();

        mat.m11 = one / (aspect * tan_half_fov);
        mat.m22 = one / (tan_half_fov);
        mat.m33 = -(zfar + znear) / (zfar - znear);
        mat.m43 = -one;
        mat.m34 = -(two * zfar * znear) / (zfar - znear);

        Perspective {
            mat: mat,
        }
    }

    pub fn aspect(&self) -> N {
        unsafe {
            self.mat.at_fast((2, 2)) / self.mat.at_fast((1, 1))
        }
    }

    pub fn set_aspect(&mut self, aspect: N) {
        unsafe {
            let one_tan_half_fov = self.mat.at_fast((2, 2));
            self.mat.set_fast((1, 1), one_tan_half_fov / aspect);
        }
    }

    pub fn as_mat<'a>(&'a self) -> &'a Mat4<N> {
        &self.mat
    }

}

impl<N> Perspective<N>
    where N: BaseFloat + Clone
{
    pub fn to_mat(&self) -> Mat4<N> {
        self.mat.clone()
    }
}

impl AsUniformValue for Perspective<f32> {
    fn as_uniform_value(&self) -> UniformValue {
        let val = self.to_mat();
        UniformValue::Mat4(*val.as_array())
    }
}

impl UniformBlock for Perspective<f32> {
    fn matches(layout: &glium::program::BlockLayout, base_offset: usize)
        -> Result<(), glium::uniforms::LayoutMismatchError>
    {
        use glium::program::BlockLayout;
        use glium::uniforms::LayoutMismatchError;
        use glium::uniforms::UniformType;

        if let &BlockLayout::BasicType { ty, offset_in_buffer } = layout {
            if ty != UniformType::FloatMat4 {
                return Err(LayoutMismatchError::TypeMismatch {
                    expected: ty,
                    obtained: UniformType::FloatMat4,
                });
            }

            if offset_in_buffer != base_offset {
                return Err(LayoutMismatchError::OffsetMismatch {
                    expected: offset_in_buffer,
                    obtained: base_offset,
                });
            }

            Ok(())
        } else {
            Err(LayoutMismatchError::LayoutMismatch {
                expected: layout.clone(),
                obtained: BlockLayout::BasicType {
                    ty: UniformType::FloatMat4,
                    offset_in_buffer: base_offset,
                }
            })
        }
    }

    #[inline]
    fn build_layout(base_offset: usize) -> glium::program::BlockLayout {
        use glium::program::BlockLayout;
        use glium::uniforms::UniformType;

        BlockLayout::BasicType {
            ty: UniformType::FloatMat4,
            offset_in_buffer: base_offset,
        }
    }
}
pub fn radians<N>(n: N) -> N
    where N: BaseFloat
{
    n * (N::pi() / <N as Cast<f64>>::from(180.0))
}

pub fn degrees<N>(n: N) -> N
    where N: BaseFloat
{
    n * (<N as Cast<f64>>::from(180.0) / N::pi())
}

pub fn extend<T, N>(base: T, elem: N) -> T::Output
    where T: Extend<N>
{
    base.extend(elem)
}

pub fn truncate<T>(base: T) -> T::Output
    where T: Truncate
{
    base.truncate()
}

