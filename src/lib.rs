//! `gel` is an extension of sebcrozet's `nalgebra` crate, designed to facilitate OpenGL
//! mathematics.

extern crate num;
extern crate nalgebra;
extern crate glium;
extern crate rustc_serialize;

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

        unsafe {
            result.set_fast((0, 0), s.x);
            result.set_fast((1, 0), s.y);
            result.set_fast((2, 0), s.z);
            result.set_fast((0, 1), u.x);
            result.set_fast((1, 1), u.y);
            result.set_fast((2, 1), u.z);
            result.set_fast((0, 2), -f.x);
            result.set_fast((1, 2), -f.y);
            result.set_fast((2, 2), -f.z);
            result.set_fast((3, 0), -(s.dot(camera)));
            result.set_fast((3, 1), -(u.dot(camera)));
            result.set_fast((3, 2), f.dot(camera));
        }

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
        unsafe {
            mat.set_fast((0, 0), one / (aspect * tan_half_fov));
            mat.set_fast((1, 1), one / (tan_half_fov));
            mat.set_fast((2, 2), -(zfar + znear) / (zfar - znear));
            mat.set_fast((2, 3), -one);
            mat.set_fast((3, 2), -(two * zfar * znear) / (zfar - znear));
        }

        Perspective {
            mat: mat,
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
