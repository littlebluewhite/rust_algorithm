pub fn remove_outer_parentheses(s: String) -> String {
    let mut depth = 0;
    let mut ans = String::with_capacity(s.len());
    for &u in s.as_bytes() {
        if u == b'(' {
            if depth > 0 {
                ans.push(u as char);
            }
            depth += 1;
        } else {
            depth -= 1;
            if depth > 0 {
                ans.push(u as char);
            }
        }
    }
    ans
}
