pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut i = 0usize;
    let mut j = 0usize;
    while i < nums1.len() && j < nums2.len(){
        match nums1[i].cmp(&nums2[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => return nums1[i],
        }
    }
    -1
}