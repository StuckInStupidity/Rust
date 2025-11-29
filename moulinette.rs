#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug, PartialEq)]
struct Matrix2x2 {
    a11: f64, a12: f64,
    a21: f64, a22: f64,
}
impl Matrix2x2 {
    fn inverse(&self) -> Option<Matrix2x2> {
        let det = self.a11 * self.a22 - self.a12 * self.a21;
        if det == 0.0 {
            return None;
        }
        let inv_det = 1.0 / det;
        Some(Matrix2x2 {
            a11:  self.a22 * inv_det,
            a12: -self.a12 * inv_det,
            a21: -self.a21 * inv_det,
            a22:  self.a11 * inv_det,
        })
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)] //run cargo test in the terminal
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert_eq!(true, larger.can_hold(&smaller));

        //for an idiomatic boolean check
        //assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn inverse_of_simple_matrix() {
        let m = Matrix2x2 { a11: 4.0, a12: 7.0, a21: 2.0, a22: 6.0 };
        let inv = m.inverse().unwrap();

        let expected = Matrix2x2 {
            a11: 0.6000000000000001,  a12: -0.7000000000000001,
            a21: -0.2, a22: 0.4,
        };

        assert_eq!(expected, inv);
    }

    #[test]
    fn singular_matrix_has_no_inverse() {
        let m = Matrix2x2 { a11: 1.0, a12: 2.0, a21: 2.0, a22: 4.0 };
        assert_eq!(None, m.inverse());
    }

}