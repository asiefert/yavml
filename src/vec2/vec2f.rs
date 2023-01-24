use core::ops::*;

use super::vec2d::Vec2d;
use super::vec2i::Vec2i;
/// An integer-holding vector with 2 values
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec2f {
    ///The x value of the vector
    pub x: f32,
    ///The y value of the vector
    pub y: f32,
}

impl Vec2f {
    /// Vector of all zeros
    pub const ZERO: Self = Self::splat(0.0);
    /// Vector of all ones
    pub const ONE: Self = Self::splat(1.0);
    /// Vector of all negative ones
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// A unit vector pointing along positive y
    pub const UP: Self = Self::new(0.0, 1.0);
    /// A unit vector pointing along negative y
    pub const DOWN: Self = Self::new(0.0, -1.0);
    /// A unit vector pointing along negative x
    pub const LEFT: Self = Self::new(1.0, 0.0);
    /// A unit vector pointing along positive x
    pub const RIGHT: Self = Self::new(0.0, 1.0);

    /// Creates a new vector
    /// # Arguments
    ///
    /// * `x` - An `f32` that holds the x value of the vector
    ///
    /// * `y` - A `f32` that holds the y value of the vector
    ///
    /// # Examples:
    ///
    /// ```
    /// use yavml::vec2::vec2f::Vec2f;
    /// let vector = Vec2f::new(1.0,1.0);
    /// ```
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    /// Creates a Vector with all elements set to `val`
    ///
    /// # Arguments
    ///
    /// * `val` - An `f32` that will hold the `x` and `y` values of the vector
    ///
    /// # Examples:
    /// ```
    /// use yavml::vec2::vec2f::Vec2f;
    /// let vector = Vec2f::splat(2.0);
    /// assert_eq!(vector,Vec2f::new(2.0,2.0));
    /// ```
    pub const fn splat(val: f32) -> Self {
        Self::new(val, val)
    }
    /// Create a new Vector from an 2 item-length array
    pub const fn from_arr(arr: [f32; 2]) -> Self {
        Self::new(arr[0], arr[1])
    }

    /// Create an array from a Vector's `x` and `y` values
    pub const fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }

    pub const fn as_vec2i(&self) -> Vec2i {
        todo!()
    }

    pub const fn as_vec2d(&self) -> Vec2d {
        todo!()
    }
    /// Returns the dot product of the `self` and `rhs`
    ///
    /// # Arguments
    ///
    /// * `self` - The first `Vec2f`
    ///
    /// * `rhs` - The second `Vec2f`
    ///
    /// # Examples:
    /// ```
    /// use yavml::vec2::vec2f::Vec2f;
    /// let vector1 = Vec2f::new(2.0,3.0);
    /// let vector2 = Vec2f::new(4.0,5.0);
    /// assert_eq!(vector1.dot(vector2),23.0)
    /// ```
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Returns the cross product of `self` and `rhs`
    ///
    /// #Arguments
    ///
    /// * `self` - The first `Vec2f`
    ///
    /// * `rhs` - The Second `Vec2f`
    ///
    /// # Examples:
    /// ```
    /// use yavml::vec2::vec2f::Vec2f;
    /// let vector1 = Vec2f::new(2.0,3.0);
    /// let vector2 = Vec2f::new(4.0,5.0);
    /// assert_eq!(vector1.cross(vector2),-2.0)
    pub fn cross(self, rhs: Self) -> f32 {
        self.x * rhs.y - self.y * rhs.x
    }

    /// Sets the x and y value of a vector
    ///
    /// #Arguments
    ///
    /// * `self` - The Vector being set
    pub fn set(&mut self, new_x: f32, new_y: f32) {
        self.x = new_x;
        self.y = new_y;
    }

    /// Returns the length of the vector `self`
    pub fn length(self) -> f64 {
        ((self.dot(self)) as f64).sqrt()
    }
}

/// Addition of vectors
impl Add<Vec2f> for Vec2f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

///Addition Assignments of vectors
impl AddAssign<Vec2f> for Vec2f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// Subtraction of vectors
impl Sub<Vec2f> for Vec2f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
/// Subtraction Assignments of vectors
impl SubAssign<Vec2f> for Vec2f {
    fn sub_assign(&mut self, rhs: Vec2f) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/// Multiplication of vectors
impl Mul<Vec2f> for Vec2f {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

/// Multiplication of a vector by an `f32`
impl Mul<f32> for Vec2f {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
/// Multiplication assignment of Vectors
impl MulAssign<Vec2f> for Vec2f {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

/// Multiplication assignment of a vector by an `f32`
impl MulAssign<f32> for Vec2f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

/// Division of a vector
impl Div<Vec2f> for Vec2f {
    type Output = Self;
    fn div(self, rhs: Vec2f) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}
/// Division of a vector by an `f32`
impl Div<f32> for Vec2f {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<Vec2f> for Vec2f {
    fn div_assign(&mut self, rhs: Vec2f) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl DivAssign<f32> for Vec2f {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
