pub fn maximum_length_substring(s: String) -> i32 {
    let mut freq = [0i32; 26];
    let b = s.as_bytes();
    let n = b.len();
    let mut best = 0;
    let mut l = 0usize;
    for r in 0..n{
        freq[(b[r] - b'a') as usize] += 1;
        while freq[(b[r] - b'a') as usize] > 2 {
            freq[(b[l] - b'a') as usize] -= 1;
            l += 1;
        }
        best = best.max(r - l + 1);
    }
    best as i32
}