pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut ans: Vec<Vec<i32>> = Vec::new();
    let n = arr.len();
    arr.sort_unstable();
    let mut min_diff = i32::MAX;
    for i in 0..n-1{
        let diff = arr[i+1] - arr[i];
        if diff < min_diff{
            min_diff = diff;
        }
    }
    for i in 0..n-1{
        if arr[i+1] - arr[i] == min_diff{
            ans.push(vec![arr[i], arr[i+1]]);
        }
    }
    ans
}