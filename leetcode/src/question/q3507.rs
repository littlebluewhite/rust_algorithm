// [5,2,3,1]
pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
    let mut ans = 0i32;
    let mut index = 0usize;
    while is_nondecreasing(&nums) != true{
        let mut min_sum = i32::MAX;
        for i in 0..nums.len()-1{
            let sum = nums[i] + nums[i+1];
            if sum < min_sum{
                min_sum = sum;
                index = i;
            }
        }
        nums[index] = min_sum;
        nums.remove(index+1);
        ans += 1;
    }
    ans
}

fn is_nondecreasing(nums: &Vec<i32>) -> bool{
    let mut ans = true;
    for i in 0..nums.len()-1{
        if nums[i] > nums[i+1]{
            ans = false;
            break;
        }
    }
    ans
}
