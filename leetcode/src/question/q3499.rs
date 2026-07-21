pub fn max_active_sections_after_trade(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    let mut new = vec![0; n + 2];
    let mut count = 0;
    new[0] = 1;
    new[n + 1] = 1;
    for i in 0..n {
        let c = b[i] - b'0';
        if c == 1 {
            count += 1;
        }
        new[i + 1] = c;
    }
    let mut pre1: (u8, i32) = (3, 0);
    let mut pre2: (u8, i32) = (3, 0);
    let mut i = 0;
    let mut best_add = 0;
    while i < n + 2 {
        let ch = new[i];
        let mut len = 1;
        i += 1;
        while i < n + 2 && new[i] == ch {
            len += 1;
            i += 1;
        }
        if ch == 0 && pre2.0 == 0 && pre1.0 == 1 {
            best_add = best_add.max(pre2.1 + len);
        }
        pre2 = pre1;
        pre1 = (ch, len);
    }
    count + best_add
}
