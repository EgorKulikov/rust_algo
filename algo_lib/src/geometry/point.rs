use crate::geometry::base::Base;
use crate::geometry::geometry_utils::canonize_angle;
use crate::geometry::line::Line;
use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::numbers::num_traits::algebra::Zero;
use crate::numbers::real::{IntoReal, Real};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Zero> Point<T> {
    pub fn origin() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

impl<T: Base> Point<T> {
    pub fn square_dist_point(&self, p: Self) -> T {
        let delta = *self - p;
        delta * delta
    }

    pub fn line(&self, p: Self) -> Line<T> {
        let a = self.y - p.y;
        let b = p.x - self.x;
        Line::new(a, b, T::zero() - a * self.x - b * self.y)
    }
}

impl<T: Base + IntoReal> Point<T> {
    pub fn dist_point(&self, p: Self) -> Real {
        self.square_dist_point(p).into_real().sqrt()
    }
}

impl Point<Real> {
    pub fn from_polar(r: Real, alpha: Real) -> Self {
        Self::new(r * alpha.cos(), r * alpha.sin())
    }

    pub fn angle(&self) -> Real {
        Real::atan2(self.y, self.x)
    }

    pub fn angle_to(&self, p: Self) -> Real {
        canonize_angle(p.angle() - self.angle()).abs()
    }
}

impl<T: AddAssign> Add for Point<T> {
    type Output = Point<T>;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: AddAssign> AddAssign for Point<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: SubAssign> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<T: SubAssign> SubAssign for Point<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Point<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: MulAssign + Copy> Mul<T> for Point<T> {
    type Output = Point<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Point<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: DivAssign + Copy> Div<T> for Point<T> {
    type Output = Point<T>;

    fn div(mut self, rhs: T) -> Self::Output {
        self /= rhs;
        self
    }
}

impl<T: Mul<Output = U>, U: Add> Mul for Point<T> {
    type Output = U::Output;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl<T: Writable> Writable for Point<T> {
    fn write(&self, output: &mut Output) {
        self.x.write(output);
        ' '.write(output);
        self.y.write(output);
    }
}

impl<T: Readable> Readable for Point<T> {
    fn read(input: &mut Input) -> Self {
        let x = input.read();
        let y = input.read();
        Self::new(x, y)
    }
}
