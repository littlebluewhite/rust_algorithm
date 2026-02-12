pub fn longest_balanced(nums: Vec<i32>) -> i32 {
    const MAX_VAL: usize = 100_000;
    let n = nums.len();
    let mut seen = vec![0; MAX_VAL+1];
    let mut version = 0;
    let mut ans = 0;
    for i in 0..n{
        version +=1;
        let mut distinct_odd = 0;
        let mut distinct_even = 0;
        for j in i..n{
            let val = nums[j] as usize;
            if seen[val] != version{
                seen[val] = version;
                if val % 2 == 0{
                    distinct_even += 1;
                }else{
                    distinct_odd += 1;
                }
            }
            if distinct_even == distinct_odd{
                ans = ans.max(j-i+1);
            }
        }
    }
    ans as i32
}