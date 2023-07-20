use crate::geometry::geometry_utils::canonize_angle;
use crate::geometry::line::Line;
use crate::io::input::{Input, Readable};
use crate::io::output::{Output, Writable};
use crate::numbers::num_traits::real::RealTrait;
use crate::numbers::num_traits::ring::Ring;
use crate::numbers::num_traits::zero_one::ZeroOne;
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

impl<T: ZeroOne> Point<T> {
    pub fn origin() -> Self {
        Self::new(T::zero(), T::zero())
    }
}

impl<T: Ring + Copy> Point<T> {
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

impl<T: RealTrait> Point<T> {
    pub fn from_polar(r: T, alpha: T) -> Self {
        Self::new(r * alpha.cos(), r * alpha.sin())
    }

    pub fn dist_point(&self, p: Self) -> T {
        self.square_dist_point(p).sqrt()
    }

    pub fn same(&self, p: Self) -> bool {
        (self.x - p.x).abs() < T::epsilon() && (self.y - p.y).abs() < T::epsilon()
    }

    pub fn angle(&self) -> T {
        T::atan2(self.y, self.x)
    }

    pub fn angle_to(&self, p: Self) -> T {
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
