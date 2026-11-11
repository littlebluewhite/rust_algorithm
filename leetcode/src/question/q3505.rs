use std::collections::BTreeSet;

pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
    let width = x as usize;
    let k = k as usize;
    let n = nums.len();
    let mut lower: BTreeSet<(i32, usize)> = BTreeSet::new();
    let mut upper: BTreeSet<(i32, usize)> = BTreeSet::new();
    let lower_count = (width+1)/2;
    let mut lower_sum = 0i64;
    let mut total_sum = 0i64;
    let mut cost: Vec<i64> = Vec::with_capacity(n-width+1);
    for (right, &value) in nums.iter().enumerate() {
        if right >= width {
            let left = right - width;
            let outgoing = (nums[left], left);
            if lower.remove(&(outgoing)) {
                lower_sum -= outgoing.0 as i64;
            } else {
                upper.remove(&(outgoing));
            }
            total_sum -= outgoing.0 as i64;
        }

        let incoming = (value, right);
        if lower.last().is_some_and(|&boundary| incoming <= boundary) {
            lower.insert(incoming);
            lower_sum += value as i64;
        } else {
            upper.insert(incoming);
        }
        total_sum += value as i64;

        if right + 1 < width {
            continue;
        }
        while lower.len() < lower_count {
            let item = upper.pop_first().unwrap();
            lower.insert(item);
            lower_sum += item.0 as i64;
        }
        while lower.len() > lower_count {
            let item = lower.pop_last().unwrap();
            upper.insert(item);
            lower_sum -= item.0 as i64;
        }


        let median = lower.last().unwrap().0 as i64;
        let left_cost = median * lower.len() as i64 - lower_sum;
        let right_cost = total_sum - lower_sum - median * (upper.len() as i64);
        cost.push(left_cost + right_cost);
    }

    let mut prev = vec![0i64;n+1];
    let mut cur = vec![i64::MAX;n+1];
    for count in 1..=k{
        cur.fill(i64::MAX);
        for end in width*count..=n{
            let start = end-width;
            let take = prev[start]+cost[start];
            cur[end] = cur[end-1].min(take);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[n]
}