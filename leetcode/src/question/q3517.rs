pub fn smallest_palindrome(s: String) -> String {
    let b = s.as_bytes();
    let n = b.len();
    let mut count: Vec<i32> = vec![0; 26];
    for i in 0..n {
        count[(b[i] - b'a') as usize] += 1;
    }
    let mut ans = vec![0u8; n];
    let mut cur = 0;
    for i in 0..26 {
        let mut ch_count = count[i];
        let ch = b'a' + i as u8;
        if ch_count % 2 == 1 {
            ans[n / 2] = ch;
            ch_count -= 1;
        }
        for _ in 0..ch_count / 2 {
            ans[cur] = ch;
            ans[n - 1 - cur] = ch;
            cur += 1;
        }
    }
    String::from_utf8(ans).unwrap()
}