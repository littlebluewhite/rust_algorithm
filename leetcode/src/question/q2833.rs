pub fn furthest_distance_from_origin(moves: String) -> i32 {
    let mut position = 0i32;
    let mut s_count = 0i32;
    for i in moves.as_bytes() {
        match i {
            b'L' => position += 1,
            b'R' => position -= 1,
            b'_' => s_count += 1,
            _ => unreachable!(),
        }
    }
    position.abs() + s_count
}
