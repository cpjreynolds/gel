//! `gel` is an extension of sebcrozet's `nalgebra` crate, designed to facilitate OpenGL
//! mathematics.

extern crate num;
extern crate nalgebra;
extern crate glium;
extern crate rustc_serialize;

use std::mem;
use std::ptr;

use glium::program::BlockLayout;
use glium::uniforms::{
    AsUniformValue,
    UniformValue,
    UniformBlock,
    LayoutMismatchError,
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

macro_rules! impl_extend {
    ($t_in:ident, $t_out:ident, {$($compN:ident),*} + $ext:ident) => {
        impl<N> Extend<N> for $t_in<N>
            where N: Copy
        {
            type Output = $t_out<N>;

            fn extend(&self, elem: N) -> Self::Output {
                unsafe {
                    let mut res: Self::Output = mem::uninitialized();
                    $(ptr::write(&mut res.$compN, self.$compN);)*
                    ptr::write(&mut res.$ext, elem);
                    res
                }
            }
        }
    }
}

macro_rules! impl_truncate {
    ($t_in:ident, $t_out:ident, {$($compN:ident),*}) => {
        impl<N> Truncate for $t_in<N>
            where N: Copy
        {
            type Output = $t_out<N>;

            fn truncate(&self) -> Self::Output {
                $t_out::new($(self.$compN),*)
            }
        }
    }
}

impl_extend!(Vec0, Vec1, {} + x);
impl_extend!(Vec1, Vec2, {x} + y);
impl_extend!(Vec2, Vec3, {x, y} + z);
impl_extend!(Vec3, Vec4, {x, y, z} + w);
impl_extend!(Vec4, Vec5, {x, y, z, w} + a);
impl_extend!(Vec5, Vec6, {x, y, z, w, a} + b);

impl_extend!(Pnt0, Pnt1, {} + x);
impl_extend!(Pnt1, Pnt2, {x} + y);
impl_extend!(Pnt2, Pnt3, {x, y} + z);
impl_extend!(Pnt3, Pnt4, {x, y, z} + w);
impl_extend!(Pnt4, Pnt5, {x, y, z, w} + a);
impl_extend!(Pnt5, Pnt6, {x, y, z, w, a} + b);

impl_truncate!(Vec1, Vec0, {});
impl_truncate!(Vec2, Vec1, {x});
impl_truncate!(Vec3, Vec2, {x, y});
impl_truncate!(Vec4, Vec3, {x, y, z});
impl_truncate!(Vec5, Vec4, {x, y, z, w});
impl_truncate!(Vec6, Vec5, {x, y, z, w, a});

impl_truncate!(Pnt1, Pnt0, {});
impl_truncate!(Pnt2, Pnt1, {x});
impl_truncate!(Pnt3, Pnt2, {x, y});
impl_truncate!(Pnt4, Pnt3, {x, y, z});
impl_truncate!(Pnt5, Pnt4, {x, y, z, w});
impl_truncate!(Pnt6, Pnt5, {x, y, z, w, a});

impl<N> LookAt<N> for Rot3<N>
    where N: BaseFloat
{
    fn look_at(camera: &Vec3<N>, target: &Vec3<N>, up: &Vec3<N>) -> Self {
        let zaxis = (*target - *camera).normalize();
        let xaxis = zaxis.cross(up).normalize();
        let yaxis = xaxis.cross(&zaxis);

        let mat = Mat3::new(xaxis.x, xaxis.y, xaxis.z,
                            yaxis.x, yaxis.y, yaxis.z,
                            -zaxis.x, -zaxis.y, -zaxis.z);

        unsafe {
            Rot3::new_with_mat(mat)
        }
    }
}

impl<N> LookAt<N> for Iso3<N>
    where N: BaseFloat
{
    fn look_at(camera: &Vec3<N>, target: &Vec3<N>, up: &Vec3<N>) -> Self {

        let zaxis = (*target - *camera).normalize();
        let xaxis = zaxis.cross(up).normalize();
        let yaxis = xaxis.cross(&zaxis);

        let mat = Mat3::new(xaxis.x, xaxis.y, xaxis.z,
                            yaxis.x, yaxis.y, yaxis.z,
                            -zaxis.x, -zaxis.y, -zaxis.z);

        let rotmat = unsafe {
            Rot3::new_with_mat(mat)
        };

        let tx = -(camera.dot(&xaxis));
        let ty = -(camera.dot(&yaxis));
        let tz = camera.dot(&zaxis);
        let trans = Vec3::new(tx, ty, tz);

        Iso3::new_with_rotmat(trans, rotmat)
    }
}

impl<N> LookAt<N> for Mat4<N>
    where N: BaseFloat
{
    fn look_at(camera: &Vec3<N>, target: &Vec3<N>, up: &Vec3<N>) -> Self {

        let zaxis = (*target - *camera).normalize();
        let xaxis = zaxis.cross(up).normalize();
        let yaxis = xaxis.cross(&zaxis);

        let tx = -(camera.dot(&xaxis));
        let ty = -(camera.dot(&yaxis));
        let tz = camera.dot(&zaxis);

        let one = N::one();
        let zero = N::zero();

        Mat4::new( xaxis.x,  xaxis.y,  xaxis.z, tx,
                   yaxis.x,  yaxis.y,  yaxis.z, ty,
                  -zaxis.x, -zaxis.y, -zaxis.z, tz,
                   zero,     zero,     zero,    one,)

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
        let two = one + one;

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
        self.mat.m22 / self.mat.m11
    }

    pub fn set_aspect(&mut self, aspect: N) {
        self.mat.m11 = self.mat.m22 / aspect;
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
        UniformValue::Mat4(unsafe { mem::transmute(val) })
    }
}

impl UniformBlock for Perspective<f32> {
    fn matches(layout: &BlockLayout, base_offset: usize)
        -> Result<(), LayoutMismatchError>
    {
        Mat4::matches(layout, base_offset)
    }

    fn build_layout(base_offset: usize) -> glium::program::BlockLayout {
        Mat4::build_layout(base_offset)
    }
}

pub trait FloatExt: BaseFloat {
    fn radians(self) -> Self {
        self * (Self::pi() / <Self as Cast<f64>>::from(180.0))
    }

    fn degrees(self) -> Self {
        self * (<Self as Cast<f64>>::from(180.0) / Self::pi())
    }
}

impl FloatExt for f32 {}
impl FloatExt for f64 {}

pub fn radians<N>(n: N) -> N
    where N: FloatExt
{
    n.radians()
}

pub fn degrees<N>(n: N) -> N
    where N: FloatExt
{
    n.degrees()
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_extend {
        ($t_in:ident, $t_out:ident, $elt:ident) => {{
            let x = $t_in::zero();
            let mut t = $t_out::zero();
            t.$elt = one::<f32>();
            let n = x.extend(one());

            assert!(n.approx_eq(&t));
        }}
    }

    #[test]
    fn test_extend() {
        test_extend!(Vec0, Vec1, x);
        test_extend!(Vec1, Vec2, y);
        test_extend!(Vec2, Vec3, z);
        test_extend!(Vec3, Vec4, w);
        test_extend!(Vec4, Vec5, a);
        test_extend!(Vec5, Vec6, b);
    }

    #[test]
    fn test_radians() {
        let x = 360.0;
        let t = f32::two_pi();
        let n = x.radians();

        assert!(n.approx_eq(&t));
    }

    #[test]
    fn test_degrees() {
        let x = f32::two_pi();
        let t = 360.0;
        let n = x.degrees();

        assert!(n.approx_eq(&t));
    }

}

