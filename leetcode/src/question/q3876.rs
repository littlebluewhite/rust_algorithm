pub fn uniform_array(nums1: Vec<i32>) -> bool {
    let mut all_even = true;
    let mut min_odd = i32::MAX;
    let mut min_even = i32::MAX;
    for &num in nums1.iter(){
        if num % 2 == 0 {
            min_even = min_even.min(num);
        } else {
            min_odd = min_odd.min(num);
            all_even = false;
        }
    }
    all_even || min_even > min_odd
}