pub fn min_operations(s: String) -> i32 {
    let b = s.as_bytes();
    let n = b.len();
    if b.windows(2).all(|w| w[0] <= w[1]) {
        return 0;
    }
    if n == 2 {
        return -1;
    }

    let mut lo = b[0];
    let mut hi = b[0];
    for i in 0..n {
        lo = lo.min(b[i]);
        hi = hi.max(b[i]);
    }
    if b[0] == lo || b[n - 1] == hi {
        return 1;
    }
    if b[1..n - 1].iter().any(|&w| w == lo || w == hi) {
        return 2;
    }
    3
}
