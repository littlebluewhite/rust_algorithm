pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
    let n = n as i64;
    let sum: i64 = batteries.iter().map(|&x| x as i64).sum();
    let mut right = 0;
    let mut left = sum / n;
    let can = |time: i64, bat: &Vec<i32>, n: i64| -> bool {
        let mut total = 0;
        for &b in bat {
            let b = b as i64;
            total += b.min(time);
            if total >= n * time {
                return true;
            }
        }
        total >= n * time
    };
    let mut mid = (right + left + 1) / 2;
    while right < left {
        // println!("left:{} mid:{} right:{}", left, mid, right);
        if can(mid, &batteries, n) {
            right = mid;
        } else {
            left = mid - 1;
        }
        mid = (right + left + 1) / 2;
    }
    mid
}
