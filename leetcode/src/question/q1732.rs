pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut now = 0;
    for i in gain{
        now +=i;
        if now > res {
            res = now;
        }
    }
    res
}