pub fn is_trionic(nums: Vec<i32>) -> bool {
    let n = nums.len();
    let mut p = 0;
    for i in 0..n-1{
        if nums[i+1] <nums[i]{
            p = i;
            break;
        }
        if nums[i+1] == nums[i]{
            return false
        }
    }
    if p == 0 {
        return false
    }
    let mut q = 0;
    for i in p+1..n-1{
        if nums[i+1] > nums[i]{
            q = i;
            break;
        }
        if nums[i+1] == nums[i]{
            return false
        }
    }
    if q ==0 {
        return false
    }
    for i in q+1..n-1{
        if nums[i+1] < nums[i]{
            return false
        }
        if nums[i+1] == nums[i]{
            return false
        }
    }
    true
}