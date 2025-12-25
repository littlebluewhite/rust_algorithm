pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
    happiness.sort_by(|a, b| b.cmp(a));
    let mut ans = 0i64;
    let k = k as usize;
    let mut count = 0;
    while count < k {
        let val = happiness[count] as i64 - count as i64;
        ans += if val > 0 { val } else { 0 };
        count += 1;
    }
    ans
}
