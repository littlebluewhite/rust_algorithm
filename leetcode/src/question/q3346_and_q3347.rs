use std::collections::HashMap;

pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
    let m = num_operations as i64;
    let k = k as i64;

    let mut a = nums.iter().map(|x| *x as i64).collect::<Vec<_>>();
    a.sort_unstable();

    let mut c_max: i64 = 0;
    let mut l = 0usize;
    for r in 0..a.len() {
        while a[r] - a[l] > 2 * k {
            l += 1;
        }
        c_max = c_max.max((r - l + 1) as i64);
    }

    let mut freq: HashMap<i64, i64> = HashMap::new();
    for i in 0..a.len() {
        *freq.entry(a[i]).or_insert(0) += 1;
    }

    let lower_bound = |n: &Vec<i64>, target: i64| -> usize {
        let mut l = 0;
        let mut r = n.len();
        while l < r {
            let mid = (l + r) / 2;
            if n[mid] < target {
                l = mid + 1
            } else {
                r = mid
            }
        }
        l
    };
    let upper_bound = |n: &Vec<i64>, target: i64| -> usize {
        let mut l = 0;
        let mut r = n.len();
        while l < r {
            let mid = (l + r) / 2;
            if n[mid] <= target {
                l = mid + 1
            } else {
                r = mid
            }
        }
        l
    };
    let mut best_anchor = 0i64;
    for (&val, &eq) in freq.iter() {
        let l = lower_bound(&a, val - k);
        let r = upper_bound(&a, val + k);
        let cover = ((r - l) as i64).min(eq + m);
        best_anchor = best_anchor.max(cover);
    }
    let best_route = c_max.min(m);
    best_anchor.max(best_route) as i32
}
