pub fn gcd_sum(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut max_i = nums[0];
    let mut prefix_gcd = Vec::with_capacity(n);
    for i in 0..n{
        max_i = max_i.max(nums[i]);
        prefix_gcd.push(gcd(nums[i], max_i));
    }
    prefix_gcd.sort_unstable();
    let mut res = 0i64;
    for i in 0..n/2{
        res += gcd(prefix_gcd[i], prefix_gcd[n-i-1]) as i64;
    }
    res
}

fn gcd(mut a: i32, mut b: i32) -> i32{
    while b != 0{
        let r = a % b;
        a = b;
        b = r;
    }
    a
}