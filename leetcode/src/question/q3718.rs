use std::collections::HashSet;

pub fn missing_multiple(nums: Vec<i32>, mut k: i32) -> i32 {
    let set: HashSet<i32> = nums.into_iter().collect();
    let mut a = k;
    while set.contains(&a) {
        a += k;
    }
    a
}