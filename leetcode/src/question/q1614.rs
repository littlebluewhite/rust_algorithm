pub fn max_depth(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut ans = 0;
    let mut now = 0;
    for i in 0..n{
        if b[i] == b'('{
            now += 1;
            ans = ans.max(now);
        }else if b[i] == b')'{
            now -= 1;
        }
    }
    ans
}