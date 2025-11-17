pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
    let mut places = i32::MAX;
    for i in 0..nums.len(){
        if nums[i] == 1{
            if places < k{
                return false;
            }
            places = 0;
        }else{
            if places == i32::MAX{
                continue;
            }
            places +=1;
        }
    }
    true
}