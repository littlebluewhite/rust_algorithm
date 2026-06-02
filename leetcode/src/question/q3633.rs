pub fn earliest_finish_time(
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
) -> i32 {
    let mut ans = i32::MAX;
    let n = land_start_time.len();
    let m = water_start_time.len();
    let mut l_first = i32::MAX;
    for i in 0..n {
        let sum = land_start_time[i] + land_duration[i];
        l_first = l_first.min(sum);
    }
    for i in 0..m {
        let s = (water_start_time[i] - l_first).max(0) + water_duration[i];
        ans = ans.min(s + l_first);
    }
    let mut w_first = i32::MAX;
    for i in 0..m {
        let sum = water_start_time[i] + water_duration[i];
        w_first = w_first.min(sum);
    }
    for i in 0..n {
        let s = (land_start_time[i] - w_first).max(0) + land_duration[i];
        ans = ans.min(s + w_first);
    }
    ans
}
