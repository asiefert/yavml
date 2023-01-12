mod tests {
    use super::super::*;

    mod vec2i_tests {

        use super::vec2i::*;

        /// Tests creating a new `Vec2i`
        #[test]
        fn test_vec2i_new() {
            assert_eq!(Vec2i {x: 1, y: 1}, Vec2i::ONE);
        }

        /// Tests creating a `Vec2i` with all values splatted to 0
        #[test]
        fn test_vec2i_new_splatted() {
            assert_eq!(Vec2i::splat(0), Vec2i::ZERO)
        }

        /// Tests calculating the dot product of two `Vec2i`
        #[test]
        fn test_vec2i_dot_product() {
            let vec1 = Vec2i::new(5, 8);
            let vec2 = Vec2i::new(7, 8);
            assert_eq!(vec1.dot(vec2), 99);
        }

        /// Tests calculating the cross product of two `Vec2i`
        #[test]
        fn test_vec2i_cross_product() {
            let vec1 = Vec2i::new(7, 8);
            let vec2 = Vec2i::new(5, 8);
            assert_eq!(vec1.cross(vec2), 16)
        }

        /// Tests calculating the length of a vector
        #[test]
        fn test_vec2i_length() {
            assert_eq!(Vec2i {x:3, y:4}.length(), 5.0);
        }

        /// Tests setting x and y of a vector to new values
        #[test]
        fn test_vec2i_set() {
            let mut vec = Vec2i::new(1,2);
            vec.set(2, 3);
            assert_eq!(vec,Vec2i::new(2, 3));

        }

        /// Tests the addition operator for `Vec2i`
        #[test]
        fn test_vec2i_addition() {
            let vec1 = Vec2i::new(1, 1);
            let vec2 = Vec2i::new(2, 2);
            assert_eq!(vec1 + vec2, Vec2i::new(3, 3))
        }

        /// Tests the addition assignment operator for `Vec2i`
        #[test]
        fn test_vec2i_addition_assignment() {
            let mut vec1 = Vec2i::new(1, 1);
            let vec2 = Vec2i::new(9, 9);
            vec1 += vec2;
            assert_eq!(vec1, Vec2i::new(10, 10))
        }

        /// Tests the subtraction operator for `Vec2i`
        #[test]
        fn test_vec2i_subtract_two_vec2i() {
            let vec1 = Vec2i::new(2, 2);
            let vec2 = Vec2i::new(3, 3);
            assert_eq!(vec1 - vec2, Vec2i::NEG_ONE)
        }

        /// Tests the subtraction assignment operator for `Vec2i`
        #[test]
        fn test_vec2i_subtraction_assignment() {
            let mut vec1 = Vec2i::new(10, 10);
            let vec2 = Vec2i::new(1, 1);
            vec1 -= vec2;
            assert_eq!(vec1, Vec2i::new(9, 9))
        }

        /// Tests the multiplication operator for `Vec2i`
        #[test]
        fn test_vec2i_multiplication() {
            let vec1 = Vec2i::new(2, 2);
            let vec2 = Vec2i::new(2, 2);
            assert_eq!(vec1 * vec2, Vec2i::new(4, 4))
        }

        /// Tests the multiplication operator for `Vec2i` with a scalar `i32` value
        #[test]
        fn test_vec2i_multiplication_scalar() {
            let vec1 = Vec2i::new(5, 5);
            let scalar: i32 = 6;
            assert_eq!(vec1 * scalar, Vec2i::new(30, 30))
        }
        /// Tests the multiplication assignment operator for `Vec2i`
        #[test]
        fn test_vec2i_multiplication_assignment() {
            let mut vec1 = Vec2i::new(2, 2);
            let vec2 = Vec2i::new(3, 3);
            vec1 *= vec2;
            assert_eq!(vec1, Vec2i::new(6, 6));
        }
        /// Tests the multiplication assignment operator for `Vec2i` with a scalar `i32` value
        #[test]
        fn test_vec2i_multiplication_assignment_scalar() {
            let mut vec1 = Vec2i::new(2, 2);
            let scalar = 2;
            vec1 *= scalar;
            assert_eq!(vec1, Vec2i::new(4, 4));
        }

        /// Tests the division assignment operator for `Vec2i`
        #[test]
        fn test_vec2i_division_assignment() {
            let mut vec1 = Vec2i::new(6, 6);
            let vec2 = Vec2i::new(3, 3);
            vec1 /= vec2;
            assert_eq!(vec1, Vec2i::new(2, 2));
        }

        /// Tests the division assignment operator for `Vec2i` with a scalar `i32` value
        #[test]
        fn test_vec2i_division_assignment_scalar() {
            let mut vec1 = Vec2i::new(6, 6);
            let scalar = 2;
            vec1 /= scalar;
            assert_eq!(vec1, Vec2i::new(3, 3));
        }

        /// Tests that the compiler panics when a vector is divided by a vector with a value set to zero
        #[test]
        #[should_panic(expected = "attempt to divide by zero")]
        fn test_vec2i_panic_on_div_by_zero() {
            let vec1 = Vec2i::new(1, 1);
            let vec2 = Vec2i::new(0,0);
            let _ = vec1 / vec2;
        }

        /// Tests that the compiler panics when a vector is divided by zero
        #[test]
        #[should_panic(expected = "attempt to divide by zero")]
        fn test_vec2i_panic_on_div_by_zero_scalar() {
            let vector = Vec2i::new(1, 1);
            let _ = vector / 0;
        }
    }

    mod vec2f_tests {
        // use super::vec2f32::Vec2f32;
    }
}
