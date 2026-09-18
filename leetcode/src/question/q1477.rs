pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
    let n = arr.len();
    let inf = n+1;
    let mut best_left = vec![0; n];
    let mut shortest = inf;
    let mut ans = inf;
    let mut sum = 0;
    let mut left = 0;
    for right in 0..n {
        sum += arr[right];
        while sum > target {
            sum -= arr[left];
            left += 1;
        }
        if sum == target {
            let len = right - left + 1;
            if left > 0{
                ans = ans.min(best_left[left-1]+len);
            }
            shortest = shortest.min(len);
        }
        best_left[right] = shortest;
    }
    if ans == inf{
        -1
    } else {
        ans as i32
    }
}