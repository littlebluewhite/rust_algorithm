pub fn judge_circle(moves: String) -> bool {
    let moves = moves.as_bytes();
    if moves.len() % 2 != 0 {
        return false;
    }
    let mut count = (0,0);
    for &c in moves{
        match c {
            b'U' => count.0 += 1,
            b'D' => count.0 -= 1,
            b'L' => count.1 += 1,
            b'R' => count.1 -= 1,
            _ => {}
        }
    }
    if count.0 != 0 || count.1 != 0 {
        return false;
    }
    true
}