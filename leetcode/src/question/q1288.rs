use std::cmp::Ordering;

pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_by(|a, b| {
        match a[0].cmp(&b[0]){
            Ordering::Equal => b[1].cmp(&a[1]),
            other => other
        }
    });
    let n = intervals.len();
    let mut max_right = intervals[0][1];
    let mut ans = n as i32;
    for i in 1..n {
        if intervals[i][1] <= max_right{
            ans -= 1;
        }else{
            max_right = intervals[i][1];
        }
    }
    ans
}