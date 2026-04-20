pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut ans = 0usize;
    let mut j = 0usize;

    for i in 0..nums1.len(){
        if j < i{
            j=i;
        }
        while j < nums2.len() && nums2[j] >= nums1[i]{
            ans = ans.max(j-i);
            j += 1;
        }
    }
    ans as i32
}