extern crate num;
extern crate nalgebra as na;
extern crate glium;
extern crate rustc_serialize;

pub use num::{
    One,
    Zero,
};

pub use na::*;

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

pub trait LookAt<N> {
    fn look_at(camera: &Vec3<N>, target: &Vec3<N>, up: &Vec3<N>) -> Self;
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

