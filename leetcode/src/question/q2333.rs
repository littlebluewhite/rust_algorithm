pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
    let mut budget = (k1 + k2) as i64;
    let mut total = 0i64;
    let mut max_diff = 0usize;
    for (&a, &b) in nums1.iter().zip(&nums2){
        let diff = (a - b).abs() as usize;
        max_diff = max_diff.max(diff);
        total += diff as i64;
    }
    if total <= budget {
        return 0;
    }
    let mut freq = vec![0i64; max_diff + 1];
    for (&a, &b) in nums1.iter().zip(&nums2){
        let diff = (a - b).abs() as usize;
        freq[diff] += 1;
    }
    for diff in (1..=max_diff).rev() {
        let take = freq[diff].min(budget);
        budget -= take;
        freq[diff] -= take;
        freq[diff - 1] += take;
        if budget == 0 {
            break;
        }
    }
    freq.into_iter().enumerate().map( |(diff, count)| {
        count*diff as i64*diff as i64
    }).sum()
}
