use std::collections::HashSet;

pub fn sum_counts(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    for left in 0..n {
        let mut set: HashSet<i32> = HashSet::new();
        for right in left..n {
            set.insert(nums[right]);
            let len = set.len() as i32;
            ans += len * len;
        }
    }
    ans
}
