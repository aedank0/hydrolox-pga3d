pub mod line;
pub mod motor;
pub mod plane;
pub mod point;
pub mod prelude;

#[cfg(not(feature = "f64"))]
pub type Float = f32;
#[cfg(feature = "f64")]
pub type Float = f64;

#[cfg(test)]
mod test {
    use motor::Motor;
    use point::Point;

    use super::*;

    #[cfg(not(feature = "f64"))]
    use std::f32::consts as float_consts;
    #[cfg(feature = "f64")]
    use std::f64::consts as float_consts;

    #[test]
    fn motor_translate() {
        let move_two_right = Motor::from_translation(2.0, 0.0, 0.0);
        let two_right = Point::new(2.0, 0.0, 0.0, 1.0);
        let should_be_same = move_two_right.transform(Point::ZERO);

        assert!(two_right.is_close(should_be_same));
    }

    #[test]
    fn motor_rotate() {
        let yaw_90_deg = Motor::from_rotation_around_axis(0.0, 1.0, 0.0, float_consts::PI * 0.5);
        let one_fwd_one_down = Point::new(0.0, -1.0, -1.0, 1.0);
        let should_be_same = yaw_90_deg.transform(Point::new(1.0, -1.0, 0.0, 1.0));

        assert!(one_fwd_one_down.is_close(should_be_same));
    }

    #[test]
    fn motor_combine() {
        let move_two_right = Motor::from_translation(2.0, 0.0, 0.0);
        let move_two_forward = Motor::from_translation(0.0, 0.0, -2.0);
        let yaw_270_deg = Motor::from_rotation_around_axis(0.0, 1.0, 0.0, float_consts::PI * 1.5);
        let should_be_same = move_two_forward.combine(yaw_270_deg);

        let p1 = move_two_right.transform(Point::default());
        let p2 = should_be_same.transform(Point::default());

        assert!(p1.is_close(p2));
    }

    #[test]
    fn motor_combination_bug() {
        let translate = Motor::from_translation(-1.0, -1.0, -1.0);
        println!("translate: {:#?}\n", translate);
        let rotate = Motor::from_rotation_around_axis(0.0, 1.0, 0.0, -float_consts::PI * 0.5);
        println!("rotate: {:#?}\n", rotate);

        let motor = translate.combine(rotate);
        println!("final motor: {:#?}\n", motor);

        let separate_point = rotate.transform(translate.transform(Point::FORWARD));
        println!("separate point: {:#?}\n", separate_point);
        let combined_point = motor.transform(Point::FORWARD);
        println!("combined point: {:#?}\n", combined_point);

        assert!(
            separate_point.is_close(Point::from_position(2.0, -1.0, -1.0)),
            "Separated motors"
        );
        assert!(
            combined_point.is_close(Point::from_position(2.0, -1.0, -1.0)),
            "Combined motors"
        );
    }

    #[test]
    fn motor_inverse() {
        //Camera is right, up, and back, and is angled 45 degrees to the left
        let camera = Motor::from_rotation_around_axis(0.0, 1.0, 0.0, float_consts::PI * 0.25)
            .combine(Motor::from_translation(1.0, 1.0, 1.0));
        assert!(
            camera
                .transform(Point::ZERO)
                .is_close(Point::from_position(1.0, 1.0, 1.0)),
            "Sanity check"
        );

        let point = Point::from_position(0.0, 1.0, 0.0);

        assert!(camera
            .inverse()
            .transform(point)
            .is_close(Point::from_position(0.0, 0.0, -(2.0 as Float).sqrt())));
    }

    #[test]
    fn motor_from_euler_angles() {
        let x = (30.0 as Float).to_radians();
        let y = (60.0 as Float).to_radians();
        let z = (45.0 as Float).to_radians();
        let motor = Motor::from_euler_angles(x, y, z);
        let separate_rots = Motor::from_rotation_around_axis(0.0, 0.0, 1.0, z)
            .combine(Motor::from_rotation_around_axis(1.0, 0.0, 0.0, x))
            .combine(Motor::from_rotation_around_axis(0.0, 1.0, 0.0, y));

        let point = Point::from_position(2.0, 3.0, 4.0);

        assert!(motor
            .transform(point)
            .is_close(separate_rots.transform(point)));
    }

    #[test]
    fn motor_from_pos_rot() {
        let rot_x = (50.0 as Float).to_radians();
        let rot_y = (35.0 as Float).to_radians();
        let rot_z = (80.0 as Float).to_radians();
        let pos_x = 7.0;
        let pos_y = 3.5;
        let pos_z = 4.0;

        let motor = Motor::from_euler_pos_and_rot(pos_x, pos_y, pos_z, rot_x, rot_y, rot_z);
        let separate_motors = Motor::from_rotation_around_axis(0.0, 0.0, 1.0, rot_z)
            .combine(Motor::from_rotation_around_axis(1.0, 0.0, 0.0, rot_x))
            .combine(Motor::from_rotation_around_axis(0.0, 1.0, 0.0, rot_y))
            .combine(Motor::from_translation(pos_x, pos_y, pos_z));

        let point = Point::from_position(1.0, 0.5, 2.0);

        assert!(motor
            .transform(point)
            .is_close(separate_motors.transform(point)));
    }

    #[test]
    fn motor_translation_euler() {
        let motor = Motor::from_euler_pos_and_rot(1.0, 2.0, 3.0, 2.0, 0.7, 5.0);

        assert!(Point::from_position(1.0, 2.0, 3.0).is_close(motor.translation_euler()));
    }

    #[test]
    fn motor_factorize() {
        let motor = Motor::from_euler_pos_and_rot(3.0, 2.0, 1.0, 2.5, 3.0, 4.0);
        let (translation, rotation) = motor.factorize();

        let point = Point::from_position(0.5, 0.3, 0.7);

        //println!("{:#?}", translation.translation_euler());

        assert!(translation
            .transform(point)
            .is_close(Point::from_position(3.5, 2.3, 1.7)));
        assert!(rotation
            .transform(point)
            .is_close(Motor::from_euler_angles(2.5, 3.0, 4.0).transform(point)));
        assert!(motor
            .transform(point)
            .is_close(rotation.combine(translation).transform(point)));
    }

    #[cfg(feature = "serde")]
    #[test]
    fn json_serde() {
        let val = Motor::IDENTITY;

        let json_str = serde_json::to_string_pretty(&val).unwrap();
        let val2: Motor = serde_json::from_str(&json_str).unwrap();

        assert_eq!(val, val2);
    }

    #[cfg(feature = "bytemuck")]
    #[test]
    fn bytemuck() {
        let val = Motor::IDENTITY;
        let bytes = bytemuck::bytes_of(&val);

        assert_eq!(val, *bytemuck::from_bytes::<Motor>(bytes));
    }
}
