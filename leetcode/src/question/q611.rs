pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return 0
    }
    let mut ans = 0_i32;
    nums.sort();
    for k in (2..nums.len()) {
        let mut i = 0usize;
        let mut j = k - 1;
        while i < j {
            if nums[i] + nums[j] > nums[k] {
                ans += (j-i) as i32;
                j -= 1;
            }else{
                i += 1;
            }
        }
    }
    ans
}