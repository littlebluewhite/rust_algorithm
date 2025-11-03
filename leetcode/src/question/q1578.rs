pub fn min_cost(colors: String, mut needed_time: Vec<i32>) -> i32 {
    let mut result = 0i32;
    let b = colors.as_bytes();
    for i in 1..b.len() {
        if b[i] == b[i-1] {
            result += needed_time[i].min(needed_time[i-1]);
            needed_time[i] = needed_time[i].max(needed_time[i-1])
        }
    }
    result
}