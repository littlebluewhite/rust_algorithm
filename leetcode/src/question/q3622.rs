pub fn check_divisibility(n: i32) -> bool {
    let mut origin = n;
    let mut sum = 0;
    let mut product = 1;
    while origin > 0 {
        let r = origin % 10;
        sum += r;
        product *= r;
        origin /= 10;
    }
    n % (product + sum) == 0
}
