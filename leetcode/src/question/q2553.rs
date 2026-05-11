pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    for mut num in nums {
        let mut d = Vec::new();
        while num > 0 {
            d.push(num % 10);
            num /= 10;
        }
        d.reverse();
        ans.extend(d);
    }
    ans
}