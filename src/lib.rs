mod leetcode{
    pub mod w4;
    pub mod w10;
    pub mod w23;
}
use leetcode::w4;

pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    w4::find_median_sorted_arrays(nums1, nums2)
}

mod tests {
    #[test]
    fn it_works() {
    }
}