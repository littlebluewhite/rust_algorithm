pub fn rotate_string(s: String, goal: String) -> bool {
    let mut shift = 0;
    let s = s.as_bytes();
    let goal = goal.as_bytes();
    if s.len() != goal.len() {
        return false;
    }
    'a: while shift < s.len() {
        for i in 0..s.len() {
            if s[i] != goal[(i + shift) % s.len()] {
                shift += 1;
                continue 'a;
            }
        }
        return true;
    }
    false
}