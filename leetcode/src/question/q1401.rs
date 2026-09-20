pub fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    let px = x_center.max(x1).min(x2);
    let py = y_center.max(y1).min(y2);
    let dx = (x_center-px) as i64;
    let dy = (y_center-py) as i64;
    let r = radius as i64;
    dx.pow(2)+dy.pow(2) <= r.pow(2)
}