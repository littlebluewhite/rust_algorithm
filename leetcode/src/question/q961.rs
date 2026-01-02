use std::collections::HashSet;

pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
    let mut set:HashSet<i32> = HashSet::new();
    for num in nums{
        if set.contains(&num){
            return num;
        }
        set.insert(num);
    }
    0
}