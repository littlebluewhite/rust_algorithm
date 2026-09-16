pub fn max_palindromes(s: String, k: i32) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let k = k as usize;
    let mut ans = 0;
    let mut i = 0usize;
    while i + k < n + 1 {
        if is_palindromes(b, i, i + k - 1) {
            ans += 1;
            i += k;
        } else if i + k < n && is_palindromes(b, i, i + k) {
            ans += 1;
            i += k + 1;
        } else {
            i += 1;
        }
    }
    ans
}

fn is_palindromes(b: &[u8], mut start: usize, mut end: usize) -> bool {
    while start < end {
        if b[start] != b[end] {
            return false;
        }
        start += 1;
        end -= 1;
    }
    true
}
