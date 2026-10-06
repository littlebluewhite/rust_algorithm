pub fn longest_valid_parentheses(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut stack: Vec<i32> = Vec::new();
    stack.push(-1);
    let mut ans = 0;
    for i in 0..n{
        if b[i] == b'(' {
            stack.push(i as i32);
        } else {
            stack.pop();
        }
        if let Some(&boundary) = stack.last() {
            ans = ans.max(i as i32- boundary);
        }else{
            stack.push(i as i32);
        }
    }
    ans
}