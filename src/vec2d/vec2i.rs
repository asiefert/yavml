use core::ops::*;
/// An integer-holding vector with 2 values
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Vec2i {
    ///The x value of the vector
    pub x: i32,
    ///The y value of the vector
    pub y: i32,
}

impl Vec2i {
    /// Vector of all zeros
    pub const ZERO: Self = Self::splat(0);
    /// Vector of all ones
    pub const ONE: Self = Self::splat(1);
    /// Vector of all negative ones
    pub const NEG_ONE: Self = Self::splat(-1);

    /// A unit vector pointing along positive x
    pub const U_X: Self = Self::new(1, 0);
    /// A unit vector pointing along positive y
    pub const U_Y: Self = Self::new(0, 1);
    /// A unit vector pointing along negative x
    pub const U_NX: Self = Self::new(1, 0);
    /// A unit vector pointing along negative y
    pub const U_NY: Self = Self::new(0, 1);

    /// Creates a new vector
    /// # Arguments
    ///
    /// * `x` - An `i32` that holds the x value of the vector
    ///
    /// * `y` - A `i32` that holds the y value of the vector
    ///
    /// # Examples:
    ///
    /// ```
    /// use yavml::vec2d::vec2i::Vec2i;
    /// let vector = Vec2i::new(1,1);
    /// ```
    pub const fn new(x: i32, y: i32) -> Self {
        Vec2i { x, y }
    }
    /// Creates a Vector with all elements set to `val`
    ///
    /// # Arguments
    ///
    /// * `val` - An `i32` that will hold the `x` and `y` values of the vector
    ///
    /// # Examples:
    /// ```
    /// use yavml::vec2d::vec2i::Vec2i;
    /// let vector = Vec2i::splat(2);
    /// assert_eq!(vector,Vec2i::new(2,2));
    /// ```
    pub const fn splat(val: i32) -> Self {
        Self { x: val, y: val }
    }
    /// Returns the dot product of the `self` and `rhs`
    ///
    /// # Arguments
    ///
    /// * `self` - The first `Vec2i`
    ///
    /// * `rhs` - The second `Vec2i`
    ///
    /// # Examples:
    /// ```
    /// use yavml::vec2d::vec2i::Vec2i;
    /// let vector1 = Vec2i::new(2,3);
    /// let vector2 = Vec2i::new(4,5);
    /// assert_eq!(vector1.dot(vector2),23)
    /// ```
    pub fn dot(self, rhs: Self) -> i32 {
        self.x * rhs.x + self.y * rhs.y
    }

    /// Returns the cross product of `self` and `rhs`
    ///
    /// #Arguments
    ///
    /// * `self` - The first `Vec2i`
    ///
    /// * `rhs` - The Second `Vec2i`
    ///
    /// # Examples:
    /// ```
    /// use yavml::vec2d::vec2i::Vec2i;
    /// let vector1 = Vec2i::new(2,3);
    /// let vector2 = Vec2i::new(4,5);
    /// assert_eq!(vector1.cross(vector2),-2)
    pub fn cross(self, rhs: Self) -> i32 {
        self.x * rhs.y - self.y * rhs.x
    }

    /// Sets the x and y value of a vector
    pub fn set(mut self, new_x: i32, new_y: i32) {
        self.x = new_x;
        self.y = new_y
    }
}

/// Addition of vectors
impl Add<Vec2i> for Vec2i {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

///Addition Assignments of vectors
impl AddAssign<Vec2i> for Vec2i {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// Subtraction of vectors
impl Sub<Vec2i> for Vec2i {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
/// Subtraction Assignments of vectors
impl SubAssign<Vec2i> for Vec2i {
    fn sub_assign(&mut self, rhs: Vec2i) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

/// Multiplication of vectors
impl Mul<Vec2i> for Vec2i {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

/// Multiplication of a vector by an `i32`
impl Mul<i32> for Vec2i {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
/// Multiplication assignment of Vectors
impl MulAssign<Vec2i> for Vec2i {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

/// Multiplication assignment of a vector by an `i32`
impl MulAssign<i32> for Vec2i {
    fn mul_assign(&mut self, rhs: i32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

/// Division of a vector
impl Div<Vec2i> for Vec2i {
    type Output = Self;
    fn div(self, rhs: Vec2i) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
/// Division of a vector by an `i32`
impl Div<i32> for Vec2i {
    type Output = Self;
    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<Vec2i> for Vec2i {
    fn div_assign(&mut self, rhs: Vec2i) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl DivAssign<i32> for Vec2i {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}
