pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::with_capacity(nums.len());
    for n in nums{
        if n & 1 == 0{
            ans.push(-1);
            continue
        }
        let mut l = 0i32;
        let mut t = n;
        while t & 1 == 1{
            l += 1;
            t >>= 1;
        }
        let bit = 1 << (l-1);
        ans.push(n-bit);
    }
    ans
}