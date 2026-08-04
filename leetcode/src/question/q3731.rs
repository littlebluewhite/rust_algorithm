pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    let n = nums.len();
    nums.sort();
    for i in 0..n-1{
        if nums[i] +1 != nums[i+1] {
            for j in nums[i]+1..nums[i+1] {
                res.push(j);
            }
        }
    }
    res
}