pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
    let mut ans = nums.len() as i32;
    for (i, &n) in nums.iter().enumerate(){
        if n == target{
            ans = ans.min((i as i32-start).abs());
        }
        if ans == 0{
            return 0;
        }
    }
    ans
}