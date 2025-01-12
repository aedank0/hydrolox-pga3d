pub mod point;
pub mod line;
pub mod plane;
pub mod prelude;
pub mod motor;

#[cfg(not(feature = "f64"))]
pub type Float = f32;
#[cfg(feature = "f64")]
pub type Float = f64;

#[cfg(test)]
mod test {
    use point::Point;
    use motor::Motor;

    use super::*;

    #[test]
    fn motor_translate() {
        let move_two_right = Motor::new(0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0);
        let two_right = Point::new(2.0, 0.0, 0.0, 1.0);
        let should_be_same = move_two_right.transform(Point::default());

        assert_eq!(two_right, should_be_same);
    }

    #[test]
    fn motor_rotate() {
        let rot_motor = Motor::new(0.0, (0.5 as Float).sqrt(), 0.0, (0.5 as Float).sqrt(), 0.0, 0.0, 0.0, 0.0);
        let one_fwd = Point::new(0.0, 0.0, -1.0, 1.0);
        let should_be_same = rot_motor.transform(Point::new(1.0, 0.0, 0.0, 1.0)).round();

        assert_eq!(one_fwd, should_be_same);
    }

    #[test]
    fn motor_combine() {
        let move_two_right = Motor::new(0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 0.0);
        let move_one_right = Motor::new(0.0, 0.0, 0.0, 1.0, 0.5, 0.0, 0.0, 0.0);
        let should_be_same = move_one_right.combine(move_one_right);

        let p1 = move_two_right.transform(Point::default());
        let p2 = should_be_same.transform(Point::default());

        assert_eq!(p1, p2);
    }
    
    #[cfg(feature = "serde")]
    #[test]
    fn json_serde() {
        let val = Motor::IDENTITY;

        let json_str = serde_json::to_string_pretty(&val).unwrap();
        let val2: Motor = serde_json::from_str(&json_str).unwrap();

        assert_eq!(val, val2);
    }
}
