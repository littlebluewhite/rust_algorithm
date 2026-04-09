const MOD: i64 = 1000000007;
pub fn xor_after_queries(mut nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    for query in queries {
        let mut lo = query[0] as usize;
        let hi = query[1] as usize;
        let ki = query[2] as usize;
        let vi = query[3] as i64;
        while lo <= hi{
            nums[lo] = ((nums[lo] as i64 * vi)%MOD) as i32;
            lo += ki;
        }
    }
    let mut ans = 0;
    for i in 0..nums.len() {
        ans ^= nums[i];
    }
    ans
}