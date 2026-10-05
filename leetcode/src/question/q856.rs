pub fn score_of_parentheses(s: String) -> i32 {
    let mut score = 0;
    let mut depth = 0;
    let b = s.as_bytes();
    let n = b.len();
    for i in 0..n {
        if b[i] == b'(' {
            depth += 1;
        } else if b[i] == b')' {
            if b[i - 1] == b'(' {
                score += 1 << (depth - 1);
            }
            depth -= 1;
        }
    }
    score
}
