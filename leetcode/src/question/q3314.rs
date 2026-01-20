pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
    let n= nums.len();
    let mut ans: Vec<i32> = Vec::with_capacity(n);
    for i in nums{
        if i & 1 == 0{
            ans.push(-1);
            continue;
        }
        let mut t = i;
        let mut len = 0u32;
        while t & 1 == 1{
            len +=1;
            t >>= 1;
        }
        let bit = 1i32 << (len-1);
        ans.push(i-bit);
    }
    ans
}