
fn find_median_sorted_arrays2(mut a: Vec<i32>, mut b: Vec<i32>) -> f64 {
    // Ensure a is the smaller array
    if a.len() > b.len() {
        return find_median_sorted_arrays2(b, a);
    }
    let m = a.len();
    let n = b.len();
    let half = (m + n + 1) / 2;

    let mut left = 0;
    let mut right = m;

    while left <= right {
        let i = (left + right) / 2; // cut on a
        let j = half - i; // cut on b

        // Sentinels for out-of-bounds
        let a_left_max = if i == 0 { i32::MIN } else { a[i - 1] };
        let a_right_min = if i == m { i32::MAX } else { a[i] };

        let b_left_max = if j == 0 { i32::MIN } else { b[j - 1] };
        let b_right_min = if j == n { i32::MAX } else { b[j] };

        // Check if partition is correct
        if a_left_max <= b_right_min && b_left_max <= a_right_min {
            // Found the right cut
            let max_of_left = a_left_max.max(b_left_max) as f64;
            if (m + n) % 2 == 1 {
                return max_of_left;
            }
            let min_of_right = a_right_min.min(b_right_min) as f64;
            return (max_of_left + min_of_right) / 2.0;
        }

        // Move partition in A
        if a_left_max > b_right_min {
            // too far right in A
            right = i - 1;
        } else {
            // too far left in A
            left = i + 1;
        }
    }

    // Should never get here if inputs are valid
    unreachable!("Input arrays are not sorted or constraints violated");
}

#[cfg(test)]
mod tests {
    use super::find_median_sorted_arrays;

    #[test]
    fn example1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);
    }

    #[test]
    fn example2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
    }

    #[test]
    fn edge_one_empty() {
        let nums1: Vec<i32> = vec![];
        let nums2 = vec![5, 7, 9];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 7.0);
    }

    #[test]
    fn even_total() {
        let nums1 = vec![0, 0];
        let nums2 = vec![0, 0];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 0.0);
    }
}

/// Time complexity: O(log(min(n, m))) where n and m are the lengths of the two arrays.
/// Space complexity: O(1).
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // Ensure that nums1 is the smaller array
    if nums1.len() > nums2.len() {
        return find_median_sorted_arrays(nums2, nums1);
    }

    let m = nums1.len();
    let n = nums2.len();
    let half = (m + n + 1) / 2;
    let mut left = 0;
    let mut right = nums1.len();

    // Binary search on nums1
    while left <= right {
        let i = (left + right) / 2;
        let j = half - i;

        // Handle edge cases with sentinels
        let a_left = if i == 0 { i32::MIN } else { nums1[i - 1] };
        let a_right = if i == m { i32::MAX } else { nums1[i] };
        let b_left = if j == 0 { i32::MIN } else { nums2[j - 1] };
        let b_right = if j == n { i32::MAX } else { nums2[j] };

        // Check if partition is correct
        if a_left <= b_right && b_left <= a_right {
            // Calculate the median
            let max_of_left = a_left.max(b_left) as f64;
            if (m + n) % 2 == 1 {
                return max_of_left;
            } else {
                let min_of_right = a_right.min(b_right) as f64;
                return (max_of_left + min_of_right) / 2.0;
            }
        }

        // Adjust binary search range
        if a_left > b_right {
            right = i - 1;
        } else {
            left = i + 1;
        }
    }

    // This code should never be reached if inputs are sorted
    unreachable!("Input arrays are not sorted or constraints violated");
}