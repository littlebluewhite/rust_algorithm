pub fn minimum_deletions(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut prefix_b = vec![0; n + 1];
    let mut suffix_a = vec![0; n + 1];
    for i in 0..n {
        let mut is_b = 0;
        if b[i] == b'b' {
            is_b = 1;
        }
        prefix_b[i + 1] = prefix_b[i] + is_b
    }
    for i in (0..n).rev() {
        let mut is_a = 0;
        if b[i] == b'a' {
            is_a = 1;
        }
        suffix_a[i] = suffix_a[i + 1] + is_a
    }
    let mut ans = i32::MAX;
    for i in 0..=n {
        let all = prefix_b[i] + suffix_a[i];
        ans = ans.min(all);
    }
    ans
}
