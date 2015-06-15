use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
};

use num::{
    Zero,
    Float,
};

pub trait Vector<N>:
    Add<Self, Output=Self> +
    Add<N, Output=Self> +
    Sub<Self, Output=Self> +
    Sub<N, Output=Self> +
    Mul<N, Output=Self> +
    Div<N, Output=Self> +
    Zero
    where N: Float
{
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Hash, Debug)]
pub struct Vec3<N>
    where N: Float
{
    x: N,
    y: N,
    z: N,
}

impl<N> Vec3<N>
    where N: Float
{
    fn new(x: N, y: N, z: N) -> Vec3<N> {
        Vec3 {
            x: x,
            y: y,
            z: z,
        }
    }
}

impl<N> Add for Vec3<N>
    where N: Float
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<N> Add<N> for Vec3<N>
    where N: Float
{
    type Output = Self;

    fn add(self, rhs: N) -> Self::Output {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl<N> Sub for Vec3<N>
    where N: Float
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<N> Sub<N> for Vec3<N>
    where N: Float
{
    type Output = Self;

    fn sub(self, rhs: N) -> Self::Output {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl<N> Mul<N> for Vec3<N>
    where N: Float
{
    type Output = Self;

    fn mul(self, rhs: N) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<N> Div<N> for Vec3<N>
    where N: Float
{
    type Output = Self;

    fn div(self, rhs: N) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<N> Zero for Vec3<N>
    where N: Float
{
    fn zero() -> Self {
        Vec3 {
            x: N::zero(),
            y: N::zero(),
            z: N::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl<N> Vector<N> for Vec3<N>
    where N: Float
{
}
