pub fn best_closing_time(customers: String) -> i32 {
    let customers = customers.as_bytes();
    let n = customers.len();
    let mut suffix_n = vec![0; n + 1];
    let mut suffix_y = vec![0; n + 1];
    let mut best = i32::MAX;
    let mut ans = usize::MAX;
    for i in 0..n {
        if customers[i] == b'N' {
            suffix_n[i + 1] = suffix_n[i] + 1;
            suffix_y[i + 1] = suffix_y[i];
        } else {
            suffix_n[i + 1] = suffix_n[i];
            suffix_y[i + 1] = suffix_y[i] + 1;
        }
    }

    for i in 0..=n {
        let val = suffix_n[i] - suffix_n[0] + suffix_y[n] - suffix_y[i];
        if val < best {
            best = val;
            ans = i;
        }
    }
    ans as i32
}
