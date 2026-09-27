pub fn reverse_parentheses(s: String) -> String {
    let b = s.as_bytes();
    let n = b.len();
    let mut stack = Vec::new();
    let mut pair: Vec<usize> = vec![0; n];
    for i in 0..n {
        if b[i] == b'(' {
            stack.push(i);
        } else if b[i] == b')' {
            let j = stack.pop().unwrap();
            pair[i] = j;
            pair[j] = i;
        }
    }
    let mut ans = String::with_capacity(n);
    let mut index = 0isize;
    let mut direction = 1isize;
    while index >= 0 && index < n as isize {
        let idx = index as usize;
        if b[idx] == b'(' || b[idx] == b')' {
            index = pair[idx] as isize;
            direction = -direction;
        } else {
            ans.push(b[idx] as char);
        }
        index += direction;
    }
    ans
}
