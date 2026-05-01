use std::collections::HashMap;

pub fn distance(nums: Vec<i32>) -> Vec<i64> {
    let n = nums.len();
    let mut ans = vec![0i64; n];
    let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
    for i in 0..n {
        map.entry(nums[i]).or_default().push(i)
    }
    for group in map.values() {
        let total_sum: i64 = group.iter().map(|&x| x as i64).sum();
        let total_count = group.len();
        let mut left_sum = 0i64;
        for (idx, &position) in group.iter().enumerate() {
            let position = position as i64;
            let right_sum = total_sum - position - left_sum;
            let right_count = (total_count - idx - 1) as i64;
            let left_count = idx as i64;
            ans[position as usize] = position * left_count - left_sum + right_sum - position * right_count;
            left_sum += position;
        }
    }
    ans
}