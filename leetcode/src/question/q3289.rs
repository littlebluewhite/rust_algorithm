use std::collections::HashSet;

pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut a:HashSet<i32> = HashSet::new();
    for i in nums{
        if a.contains(&i){
            result.push(i);
        }else{
            a.insert(i);
        }
    }
    result
}