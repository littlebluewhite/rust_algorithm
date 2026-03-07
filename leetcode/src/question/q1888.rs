pub fn min_flips(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut l = 0usize;
    let mut mismatch_0 = 0i32;
    let mut ans = i32::MAX;
    for r in 0..2 * n {
        let ch = b[r % n];
        let expect_0 = if r % 2 == 0 { b'0' } else { b'1' };
        if expect_0 != ch {
            mismatch_0 += 1;
        }
        if r - l + 1 > n {
            let ch_l = b[l % n];
            let out_0 = if l % 2 == 0 { b'0' } else { b'1' };
            if out_0 != ch_l {
                mismatch_0 -= 1;
            }
            l += 1;
        }
        if r - l + 1 == n {
            let mismatch_1 = n as i32 - mismatch_0;
            ans = ans.min(mismatch_1.min(mismatch_0));

        }
    }
    ans
}
