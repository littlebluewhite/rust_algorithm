pub fn min_element(nums: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    for i in nums.into_iter(){
        let mut i = i;
        let mut value = 0;
        while i > 0 {
            value += i%10;
            i /= 10;
        }
        min = min.min(value);
    }
    min
}