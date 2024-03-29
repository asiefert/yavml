use core::ops::*;

use super::{vec2f::Vec2f, vec2i::Vec2i};
/// An integer-holding vector with 2 values
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec2d {
    ///The x value of the vector
    pub x: f64,
    ///The y value of the vector
    pub y: f64,
}

impl Vec2d {
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
    /// * `x` - An `f64` that holds the x value of the vector
    ///
    /// * `y` - A `f64` that holds the y value of the vector
    ///
    /// # Examples:
    ///
    /// ```
    /// use yavml::vec2::Vec2d;
    /// let vector = Vec2d::new(1.0,1.0);
    /// ```
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    /// Creates a Vector with all elements set to `val`
    ///
    /// # Arguments
    ///
    /// * `val` - An `f64` that will hold the `x` and `y` values of the vector
    ///
    /// # Examples:
    /// ```
    /// use yavml::vec2::Vec2d;
    /// let vector = Vec2d::splat(2.0);
    /// assert_eq!(vector,Vec2d::new(2.0,2.0));
    /// ```
    pub const fn splat(val: f64) -> Self {
        Self::new(val, val)
    }
    /// Create a new Vector from an 2 item-length array
    pub const fn from_arr(arr: [f64; 2]) -> Self {
        Self::new(arr[0], arr[1])
    }

    /// Create an array from a Vector's `x` and `y` values
    pub const fn to_array(&self) -> [f64; 2] {
        [self.x, self.y]
    }

    /// Cast a `Vec2d` double floating-point vector as a `Vec2` integer vector
    ///
    /// # Examples:
    /// ```
    /// use yavml::vec2::Vec2d;
    /// use yavml::vec2::Vec2i;
    /// assert_eq!(Vec2d::new(6.21999979019165,7.21999979019165).as_vec2i(),Vec2i{x: 6, y: 7})
    /// ```
    pub const fn as_vec2i(&self) -> Vec2i {
        Vec2i::new(self.x as i32, self.y as i32)
    }
    /// Cast a `Vec2d` double floating-point vector as a `Vec2f` floating-point vector
    ///
    /// # Examples:
    /// ```
    /// use yavml::vec2::Vec2d;
    /// use yavml::vec2::Vec2f;
    /// assert_eq!(Vec2d::new(6.21999979019165,7.21999979019165).as_vec2f(),Vec2f{x: 6.22, y: 7.22 })
    /// ```
    pub const fn as_vec2f(&self) -> Vec2f {
        Vec2f::new(self.x as f32, self.y as f32)
    }
    /// Returns the dot product of the `self` and `rhs`
    ///
    /// # Arguments
    ///
    /// * `self` - The first `Vec2d`
    ///
    /// * `rhs` - The second `Vec2d`
    ///
    /// # Examples:
    /// ```
    /// use yavml::vec2::Vec2d;
    /// let vector1 = Vec2d::new(2.0,3.0);
    /// let vector2 = Vec2d::new(4.0,5.0);
    /// assert_eq!(vector1.dot(vector2),23.0)
    /// ```
    pub fn dot(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Returns the cross product of `self` and `rhs`
    ///
    /// #Arguments
    ///
    /// * `self` - The first `Vec2d`
    ///
    /// * `rhs` - The Second `Vec2d`
    ///
    /// # Examples:
    /// ```
    /// use yavml::vec2::Vec2d;
    /// let vector1 = Vec2d::new(2.0,3.0);
    /// let vector2 = Vec2d::new(4.0,5.0);
    /// assert_eq!(vector1.cross(vector2),-2.0)
    pub fn cross(self, rhs: Self) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }

    /// Sets the x and y value of a vector
    ///
    /// #Arguments
    ///
    /// * `self` - The Vector being set
    pub fn set(&mut self, new_x: f64, new_y: f64) {
        self.x = new_x;
        self.y = new_y;
    }

    /// Returns the length of the vector `self`
    pub fn length(self) -> f64 {
        (self.dot(self)).sqrt()
    }
}

/// Addition of vectors
impl Add<Vec2d> for Vec2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

///Addition Assignments of vectors
impl AddAssign<Vec2d> for Vec2d {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// Subtraction of vectors
impl Sub<Vec2d> for Vec2d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}
/// Subtraction Assignments of vectors
impl SubAssign<Vec2d> for Vec2d {
    fn sub_assign(&mut self, rhs: Vec2d) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/// Multiplication of vectors
impl Mul<Vec2d> for Vec2d {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

/// Multiplication of a vector by an `f64`
impl Mul<f64> for Vec2d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}
/// Multiplication assignment of Vectors
impl MulAssign<Vec2d> for Vec2d {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

/// Multiplication assignment of a vector by an `f64`
impl MulAssign<f64> for Vec2d {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

/// Division of a vector
impl Div<Vec2d> for Vec2d {
    type Output = Self;
    fn div(self, rhs: Vec2d) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}
/// Division of a vector by an `f64`
impl Div<f64> for Vec2d {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl DivAssign<Vec2d> for Vec2d {
    fn div_assign(&mut self, rhs: Vec2d) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl DivAssign<f64> for Vec2d {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
