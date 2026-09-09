pub fn count_commas(mut n: i64) -> i64 {
    let mut max_commas = 0;
    let mut copy_n = n;
    while copy_n >= 1000 {
        copy_n /= 1000;
        max_commas += 1;
    }
    let mut ans = 0;
    while max_commas > 0 {
        let min_n = 10i64.pow(max_commas * 3);
        ans = ans + (n - min_n + 1) * max_commas as i64;
        n = min_n - 1;
        max_commas -= 1;
    }
    ans
}
