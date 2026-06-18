pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
    let hour_angle = (hour*30) as f64 + minutes as f64/2.0;
    let minutes_angle = (6*minutes) as f64;
    let mut angle = (hour_angle - minutes_angle).abs();
    if angle > 180.0 {
        angle = 360.0 - angle;
    }
    angle
}