pub fn find_gcd(nums: Vec<i32>) -> i32 {
    let mut min_n = i32::MAX;
    let mut max_n = i32::MIN;
    for i in 0..nums.len(){
        if nums[i] < min_n{
            min_n = nums[i];
        }
        if nums[i] > max_n{
            max_n = nums[i];
        }
    }
    gcd(min_n, max_n)
}
fn gcd(mut a: i32, mut b: i32)-> i32{
    while b != 0{
        let r = a %b;
        a = b;
        b = r;
    }
    a
}