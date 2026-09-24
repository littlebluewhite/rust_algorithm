pub fn smallest_index(nums: Vec<i32>) -> i32 {
    for (i, num) in nums.into_iter().enumerate() {
        let i = i as i32;
        let mut digits_sum = 0;
        let mut num = num;
        while num > 0 {
            digits_sum += num % 10;
            num /= 10;
        }
        if i == digits_sum {
            return i;
        }
    }
    -1
}