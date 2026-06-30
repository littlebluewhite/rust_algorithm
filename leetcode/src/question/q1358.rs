pub fn number_of_substrings(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut ans = 0;
    let mut left = 0;
    let mut freq = vec![0;3];
    for right in 0..n{
        freq[(b[right] - b'a') as usize] += 1;
        while freq[0] > 0 && freq[1] > 0 && freq[2] > 0 {
            left += 1;
            freq[(b[left - 1] - b'a') as usize] -= 1;
        }
        ans += left;
    }
    ans as i32
}