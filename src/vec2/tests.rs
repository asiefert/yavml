use super::vec2d::Vec2d;
use super::vec2f::Vec2f;
use super::vec2i::Vec2i;
mod vec2i_tests {

    use super::Vec2d;
    use super::Vec2f;
    use super::Vec2i;

    // Tests creating a new `Vec2i`
    #[test]
    fn test_vec2i_new() {
        assert_eq!(Vec2i { x: 1, y: 1 }, Vec2i::ONE);
    }

    // Tests creating a `Vec2i` with all values splatted to 0
    #[test]
    fn test_vec2i_new_splatted() {
        assert_eq!(Vec2i::splat(0), Vec2i::ZERO)
    }

    // Tests converting a `Vec2i` to an array
    #[test]
    fn test_vec2i_to_array() {
        let vec = Vec2i::new(1, 5);
        assert_eq!(vec.to_array(), [1, 5]);
    }

    // Tests converting to a `Vec2i` from an array
    #[test]
    fn test_vec2i_from_array() {
        let arr = [1, 5];
        assert_eq!(Vec2i::from_arr(arr), Vec2i::new(1, 5));
    }

    // Tests converting a `Vec2i` to `Vec2f`
    #[test]
    fn test_vec2i_as_vec2f() {
        assert_eq!(Vec2i::new(6, 7).as_vec2f(), Vec2f { x: 6.0, y: 7.0 })
    }

    // Tests converting a `Vec2i` to `Vec2d`
    #[test]
    fn test_vec2i_as_vec2d() {
        assert_eq!(Vec2i::new(6, 7).as_vec2d(), Vec2d { x: 6.0, y: 7.0 })
    }

    // Tests calculating the dot product of two `Vec2i`
    #[test]
    fn test_vec2i_dot_product() {
        let vec1 = Vec2i::new(5, 8);
        let vec2 = Vec2i::new(7, 8);
        assert_eq!(vec1.dot(vec2), 99);
    }

    // Tests calculating the cross product of two `Vec2i`
    #[test]
    fn test_vec2i_cross_product() {
        let vec1 = Vec2i::new(7, 8);
        let vec2 = Vec2i::new(5, 8);
        assert_eq!(vec1.cross(vec2), 16)
    }

    // Tests calculating the length of a vector
    #[test]
    fn test_vec2i_length() {
        assert_eq!(Vec2i { x: 3, y: 4 }.length(), 5.0);
    }

    // Tests setting x and y of a vector to new values
    #[test]
    fn test_vec2i_set() {
        let mut vec = Vec2i::new(1, 2);
        vec.set(2, 3);
        assert_eq!(vec, Vec2i::new(2, 3));
    }

    // Tests the addition operator for `Vec2i`
    #[test]
    fn test_vec2i_addition() {
        let vec1 = Vec2i::new(1, 1);
        let vec2 = Vec2i::new(2, 2);
        assert_eq!(vec1 + vec2, Vec2i::new(3, 3))
    }

    // Tests the addition assignment operator for `Vec2i`
    #[test]
    fn test_vec2i_addition_assignment() {
        let mut vec1 = Vec2i::new(1, 1);
        let vec2 = Vec2i::new(9, 9);
        vec1 += vec2;
        assert_eq!(vec1, Vec2i::new(10, 10))
    }

    // Tests the subtraction operator for `Vec2i`
    #[test]
    fn test_vec2i_subtract_two_vec2i() {
        let vec1 = Vec2i::new(2, 2);
        let vec2 = Vec2i::new(3, 3);
        assert_eq!(vec1 - vec2, Vec2i::NEG_ONE)
    }

    // Tests the subtraction assignment operator for `Vec2i`
    #[test]
    fn test_vec2i_subtraction_assignment() {
        let mut vec1 = Vec2i::new(10, 10);
        let vec2 = Vec2i::new(1, 1);
        vec1 -= vec2;
        assert_eq!(vec1, Vec2i::new(9, 9))
    }

    // Tests the multiplication operator for `Vec2i`
    #[test]
    fn test_vec2i_multiplication() {
        let vec1 = Vec2i::new(2, 2);
        let vec2 = Vec2i::new(2, 2);
        assert_eq!(vec1 * vec2, Vec2i::new(4, 4))
    }

    // Tests the multiplication operator for `Vec2i` with a scalar `i32` value
    #[test]
    fn test_vec2i_multiplication_scalar() {
        let vec1 = Vec2i::new(5, 5);
        let scalar: i32 = 6;
        assert_eq!(vec1 * scalar, Vec2i::new(30, 30))
    }
    // Tests the multiplication assignment operator for `Vec2i`
    #[test]
    fn test_vec2i_multiplication_assignment() {
        let mut vec1 = Vec2i::new(2, 2);
        let vec2 = Vec2i::new(3, 3);
        vec1 *= vec2;
        assert_eq!(vec1, Vec2i::new(6, 6));
    }
    // Tests the multiplication assignment operator for `Vec2i` with a scalar `i32` value
    #[test]
    fn test_vec2i_multiplication_assignment_scalar() {
        let mut vec1 = Vec2i::new(2, 2);
        let scalar = 2;
        vec1 *= scalar;
        assert_eq!(vec1, Vec2i::new(4, 4));
    }

    // Tests the division assignment operator for `Vec2i`
    #[test]
    fn test_vec2i_division_assignment() {
        let mut vec1 = Vec2i::new(6, 6);
        let vec2 = Vec2i::new(3, 3);
        vec1 /= vec2;
        assert_eq!(vec1, Vec2i::new(2, 2));
    }

    // Tests the division assignment operator for `Vec2i` with a scalar `i32` value
    #[test]
    fn test_vec2i_division_assignment_scalar() {
        let mut vec1 = Vec2i::new(6, 6);
        let scalar = 2;
        vec1 /= scalar;
        assert_eq!(vec1, Vec2i::new(3, 3));
    }

    // Tests that the compiler panics when a vector is divided by a vector with a value set to zero
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_vec2i_panic_on_div_by_zero() {
        let vec1 = Vec2i::new(1, 1);
        let vec2 = Vec2i::new(0, 0);
        let _ = vec1 / vec2;
    }

    // Tests that the compiler panics when a vector is divided by zero
    #[test]
    #[should_panic(expected = "attempt to divide by zero")]
    fn test_vec2i_panic_on_div_by_zero_scalar() {
        let vector = Vec2i::new(1, 1);
        let _ = vector / 0;
    }
}

mod vec2f_tests {

    use super::Vec2d;
    use super::Vec2f;
    use super::Vec2i;

    // Tests creating a new `Vec2f`
    #[test]
    fn test_vec2f_new() {
        assert_eq!(Vec2f { x: 1.0, y: 1.0 }, Vec2f::ONE);
    }

    // Tests creating a `Vec2f` with all values splatted to 0
    #[test]
    fn test_vec2f_new_splatted() {
        assert_eq!(Vec2f::splat(0.0), Vec2f::ZERO)
    }

    //Tests converting a `Vec2f` to an array
    #[test]
    fn test_vec2f_to_array() {
        let vec = Vec2f::new(1.0, 5.0);
        assert_eq!(vec.to_array(), [1.0, 5.0]);
    }

    //Tests converting to a `Vec2f` from an array
    #[test]
    fn test_vec2f_from_array() {
        let arr = [1.0, 5.0];
        assert_eq!(Vec2f::from_arr(arr), Vec2f::new(1.0, 5.0));
    }

    // Tests converting a `Vec2f` to `Vec2i`
    #[test]
    fn test_vec2f_as_vec2i() {
        assert_eq!(Vec2f::new(6.0, 7.0).as_vec2i(), Vec2i { x: 6, y: 7 })
    }

    // Tests converting a `Vec2f` to `Vec2d`
    #[test]
    fn test_vec2f_as_vec2d() {
        assert_eq!(Vec2f::new(6.0, 7.0).as_vec2d(), Vec2d { x: 6.0, y: 7.0 })
    }

    // Tests calculating the dot product of two `Vec2f`
    #[test]
    fn test_vec2f_dot_product() {
        let vec1 = Vec2f::new(5.0, 8.0);
        let vec2 = Vec2f::new(7.0, 8.0);
        assert_eq!(vec1.dot(vec2), 99.0);
    }

    // Tests calculating the cross product of two `Vec2f`
    #[test]
    fn test_vec2f_cross_product() {
        let vec1 = Vec2f::new(7.0, 8.0);
        let vec2 = Vec2f::new(5.0, 8.0);
        assert_eq!(vec1.cross(vec2), 16.0)
    }

    // Tests calculating the length of a vector
    #[test]
    fn test_vec2f_length() {
        assert_eq!(Vec2f { x: 3.0, y: 4.0 }.length(), 5.0);
    }

    // Tests setting x and y of a vector to new values
    #[test]
    fn test_vec2f_set() {
        let mut vec = Vec2f::new(1.0, 2.0);
        vec.set(2.0, 3.0);
        assert_eq!(vec, Vec2f::new(2.0, 3.0));
    }

    // Tests the addition operator for `Vec2f`
    #[test]
    fn test_vec2f_addition() {
        let vec1 = Vec2f::new(1.0, 1.0);
        let vec2 = Vec2f::new(2.0, 2.0);
        assert_eq!(vec1 + vec2, Vec2f::new(3.0, 3.0))
    }

    // Tests the addition assignment operator for `Vec2f`
    #[test]
    fn test_vec2f_addition_assignment() {
        let mut vec1 = Vec2f::new(1.0, 1.0);
        let vec2 = Vec2f::new(9.0, 9.0);
        vec1 += vec2;
        assert_eq!(vec1, Vec2f::new(10.0, 10.0))
    }

    // Tests the subtraction operator for `Vec2f`
    #[test]
    fn test_vec2f_subtract_two_vec2f() {
        let vec1 = Vec2f::new(2.0, 2.0);
        let vec2 = Vec2f::new(3.0, 3.0);
        assert_eq!(vec1 - vec2, Vec2f::NEG_ONE)
    }

    // Tests the subtraction assignment operator for `Vec2f`
    #[test]
    fn test_vec2f_subtraction_assignment() {
        let mut vec1 = Vec2f::new(10.0, 10.0);
        let vec2 = Vec2f::new(1.0, 1.0);
        vec1 -= vec2;
        assert_eq!(vec1, Vec2f::new(9.0, 9.0))
    }

    // Tests the multiplication operator for `Vec2f`
    #[test]
    fn test_vec2f_multiplication() {
        let vec1 = Vec2f::new(2.0, 2.0);
        let vec2 = Vec2f::new(2.0, 2.0);
        assert_eq!(vec1 * vec2, Vec2f::new(4.0, 4.0))
    }

    // Tests the multiplication operator for `Vec2f` with a scalar `f32` value
    #[test]
    fn test_vec2f_multiplication_scalar() {
        let vec1 = Vec2f::new(5.0, 5.0);
        let scalar: f32 = 6.0;
        assert_eq!(vec1 * scalar, Vec2f::new(30.0, 30.0))
    }
    // Tests the multiplication assignment operator for `Vec2f`
    #[test]
    fn test_vec2f_multiplication_assignment() {
        let mut vec1 = Vec2f::new(2.0, 2.0);
        let vec2 = Vec2f::new(3.0, 3.0);
        vec1 *= vec2;
        assert_eq!(vec1, Vec2f::new(6.0, 6.0));
    }
    // Tests the multiplication assignment operator for `Vec2f` with a scalar `f32` value
    #[test]
    fn test_vec2f_multiplication_assignment_scalar() {
        let mut vec1 = Vec2f::new(2.0, 2.0);
        let scalar = 2.0;
        vec1 *= scalar;
        assert_eq!(vec1, Vec2f::new(4.0, 4.0));
    }

    // Tests the division assignment operator for `Vec2f`
    #[test]
    fn test_vec2f_division_assignment() {
        let mut vec1 = Vec2f::new(6.0, 6.0);
        let vec2 = Vec2f::new(3.0, 3.0);
        vec1 /= vec2;
        assert_eq!(vec1, Vec2f::new(2.0, 2.0));
    }

    // Tests the division assignment operator for `Vec2f` with a scalar `f32` value
    #[test]
    fn test_vec2f_division_assignment_scalar() {
        let mut vec1 = Vec2f::new(6.0, 6.0);
        let scalar = 2.0;
        vec1 /= scalar;
        assert_eq!(vec1, Vec2f::new(3.0, 3.0));
    }

    // Tests that a vector with a value set to infinity is returned when dividing by a vector with a value set to zero
    #[test]
    fn test_vec2f_infinity_on_div_by_zero() {
        let vec1 = Vec2f::new(1.0, 1.0);
        let vec2 = Vec2f::new(1.0, 0.0);
        assert_eq!(vec1 / vec2, Vec2f::new(1.0, f32::INFINITY));
    }

    // Tests that a vector with all values set to INFINITY is returned when divided by a zero-scalar
    #[test]
    fn test_vec2f_infinity_on_div_by_zero_scalar() {
        let vector = Vec2f::new(1.0, 1.0);
        assert_eq!(vector / 0.0, Vec2f::splat(f32::INFINITY));
    }
}

mod vec2d_tests {

    use super::Vec2d;
    use super::Vec2f;
    use super::Vec2i;

    // Tests creating a new `Vec2d`
    #[test]
    fn test_vec2d_new() {
        assert_eq!(Vec2d { x: 1.0, y: 1.0 }, Vec2d::ONE);
    }

    // Tests creating a `Vec2d` with all values splatted to 0
    #[test]
    fn test_vec2d_new_splatted() {
        assert_eq!(Vec2d::splat(0.0), Vec2d::ZERO)
    }

    //Tests converting a `Vec2d` to an array
    #[test]
    fn test_vec2d_to_array() {
        let vec = Vec2d::new(1.0, 5.0);
        assert_eq!(vec.to_array(), [1.0, 5.0]);
    }
    //Tests converting to a `Vec2d` from an array
    #[test]
    fn test_vec2d_from_array() {
        let arr = [1.0, 5.0];
        assert_eq!(Vec2d::from_arr(arr), Vec2d::new(1.0, 5.0));
    }

    // Tests converting a `Vec2d` to `Vec2i`
    #[test]
    fn test_vec2d_as_vec2i() {
        assert_eq!(Vec2f::new(6.21999979019165, 7.21999979019165).as_vec2i(), Vec2i { x: 6, y: 7 })
    }

    // Tests converting a `Vec2d` to `Vec2f`
    #[test]
    fn test_vec2d_as_vec2f() {
        assert_eq!(Vec2d::new(6.21999979019165, 7.21999979019165).as_vec2f(), Vec2f { x: 6.22, y: 7.22 })
    }

    // Tests calculating the dot product of two `Vec2d`
    #[test]
    fn test_vec2d_dot_product() {
        let vec1 = Vec2d::new(5.0, 8.0);
        let vec2 = Vec2d::new(7.0, 8.0);
        assert_eq!(vec1.dot(vec2), 99.0);
    }

    // Tests calculating the cross product of two `Vec2d`
    #[test]
    fn test_vec2d_cross_product() {
        let vec1 = Vec2d::new(7.0, 8.0);
        let vec2 = Vec2d::new(5.0, 8.0);
        assert_eq!(vec1.cross(vec2), 16.0)
    }

    // Tests calculating the length of a vector
    #[test]
    fn test_vec2d_length() {
        assert_eq!(Vec2d { x: 3.0, y: 4.0 }.length(), 5.0);
    }

    // Tests setting x and y of a vector to new values
    #[test]
    fn test_vec2d_set() {
        let mut vec = Vec2d::new(1.0, 2.0);
        vec.set(2.0, 3.0);
        assert_eq!(vec, Vec2d::new(2.0, 3.0));
    }

    // Tests the addition operator for `Vec2d`
    #[test]
    fn test_vec2d_addition() {
        let vec1 = Vec2d::new(1.0, 1.0);
        let vec2 = Vec2d::new(2.0, 2.0);
        assert_eq!(vec1 + vec2, Vec2d::new(3.0, 3.0))
    }

    // Tests the addition assignment operator for `Vec2d`
    #[test]
    fn test_vec2d_addition_assignment() {
        let mut vec1 = Vec2d::new(1.0, 1.0);
        let vec2 = Vec2d::new(9.0, 9.0);
        vec1 += vec2;
        assert_eq!(vec1, Vec2d::new(10.0, 10.0))
    }

    // Tests the subtraction operator for `Vec2d`
    #[test]
    fn test_vec2d_subtract_two_vec2d() {
        let vec1 = Vec2d::new(2.0, 2.0);
        let vec2 = Vec2d::new(3.0, 3.0);
        assert_eq!(vec1 - vec2, Vec2d::NEG_ONE)
    }

    // Tests the subtraction assignment operator for `Vec2d`
    #[test]
    fn test_vec2d_subtraction_assignment() {
        let mut vec1 = Vec2d::new(10.0, 10.0);
        let vec2 = Vec2d::new(1.0, 1.0);
        vec1 -= vec2;
        assert_eq!(vec1, Vec2d::new(9.0, 9.0))
    }

    // Tests the multiplication operator for `Vec2d`
    #[test]
    fn test_vec2d_multiplication() {
        let vec1 = Vec2d::new(2.0, 2.0);
        let vec2 = Vec2d::new(2.0, 2.0);
        assert_eq!(vec1 * vec2, Vec2d::new(4.0, 4.0))
    }

    // Tests the multiplication operator for `Vec2d` with a scalar `f64` value
    #[test]
    fn test_vec2d_multiplication_scalar() {
        let vec1 = Vec2d::new(5.0, 5.0);
        let scalar: f64 = 6.0;
        assert_eq!(vec1 * scalar, Vec2d::new(30.0, 30.0))
    }
    // Tests the multiplication assignment operator for `Vec2d`
    #[test]
    fn test_vec2d_multiplication_assignment() {
        let mut vec1 = Vec2d::new(2.0, 2.0);
        let vec2 = Vec2d::new(3.0, 3.0);
        vec1 *= vec2;
        assert_eq!(vec1, Vec2d::new(6.0, 6.0));
    }
    // Tests the multiplication assignment operator for `Vec2d` with a scalar `f64` value
    #[test]
    fn test_vec2d_multiplication_assignment_scalar() {
        let mut vec1 = Vec2d::new(2.0, 2.0);
        let scalar = 2.0;
        vec1 *= scalar;
        assert_eq!(vec1, Vec2d::new(4.0, 4.0));
    }

    // Tests the division assignment operator for `Vec2d`
    #[test]
    fn test_vec2d_division_assignment() {
        let mut vec1 = Vec2d::new(6.0, 6.0);
        let vec2 = Vec2d::new(3.0, 3.0);
        vec1 /= vec2;
        assert_eq!(vec1, Vec2d::new(2.0, 2.0));
    }

    // Tests the division assignment operator for `Vec2d` with a scalar `f64` value
    #[test]
    fn test_vec2d_division_assignment_scalar() {
        let mut vec1 = Vec2d::new(6.0, 6.0);
        let scalar = 2.0;
        vec1 /= scalar;
        assert_eq!(vec1, Vec2d::new(3.0, 3.0));
    }

    // Tests that a vector with a value set to infinity is returned when dividing by a vector with a value set to zero
    #[test]
    fn test_vec2d_infinity_on_div_by_zero() {
        let vec1 = Vec2d::new(1.0, 1.0);
        let vec2 = Vec2d::new(1.0, 0.0);
        assert_eq!(vec1 / vec2, Vec2d::new(1.0, f64::INFINITY));
    }

    // Tests that a vector with all values set to INFINITY is returned when divided by a zero-scalar
    #[test]
    fn test_vec2d_infinity_on_div_by_zero_scalar() {
        let vector = Vec2d::new(1.0, 1.0);
        assert_eq!(vector / 0.0, Vec2d::splat(f64::INFINITY));
    }
}
