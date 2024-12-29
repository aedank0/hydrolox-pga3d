pub mod line;
pub mod plane;
pub mod point;
pub mod prelude;
pub mod transform;

#[cfg(test)]
mod test {
    use point::Point;
    use transform::Transform;

    use super::*;

    #[test]
    fn motor_translate() {
        let move_two_right = Transform::new(0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0);
        let two_right = Point::new(2.0, 0.0, 0.0, 1.0);
        let should_be_same = move_two_right.transform(Point::default());

        assert_eq!(two_right, should_be_same);
    }

    #[test]
    fn motor_rotate() {
        let rot_motor = Transform::new(0.0, 0.5f64.sqrt(), 0.0, 0.5f64.sqrt(), 0.0, 0.0, 0.0, 0.0);
        let one_fwd = Point::new(0.0, 0.0, -1.0, 1.0);
        let should_be_same = rot_motor.transform(Point::new(1.0, 0.0, 0.0, 1.0)).round();

        assert_eq!(one_fwd, should_be_same);
    }

    #[test]
    fn motor_combine() {
        let move_two_right = Transform::new(0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0);
        let move_one_right = Transform::new(0.0, 0.0, 0.0, 1.0, 0.5, 0.0, 0.0, 0.0);
        let should_be_same = move_one_right.combine(move_one_right);

        let p1 = move_two_right.transform(Point::default());
        let p2 = should_be_same.transform(Point::default());

        assert_eq!(p1, p2);
    }
}
