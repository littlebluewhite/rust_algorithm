pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
    let mut dp = [-1;3];
    dp[0] = 0;
    for num in nums{
        let mut next = dp;
        for &v in dp.iter(){
            if v > 0{
                let new_v = v + num;
                let m = (new_v%3) as usize;
                next[m] = next[m].max(new_v);
            }
        }
        dp = next;
    }
    dp[0]
}