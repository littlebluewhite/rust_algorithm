pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
    let max_value = *nums.iter().max().unwrap() as usize;
    let mut freq = vec![0i64; max_value + 1];
    for &num in &nums {
        freq[num as usize] += 1;
    }
    let mut exact = vec![0i64; max_value + 1];
    for i in (1..=max_value).rev() {
        let mut g = i;
        let mut count = 0;
        while g <= max_value {
            count += freq[g];
            g += i;
        }
        let mut pair = count * (count - 1)/2;
        g = 2 * i;
        while g <= max_value {
            pair -= exact[g];
            g += i;
        }
        exact[i] = pair;
    }
    let mut prefix = vec![0i64; max_value + 1];
    for i in 1..=max_value {
        prefix[i] = prefix[i - 1] + exact[i];
    }
    queries
        .iter()
        .map(|&query| {
            let target = query + 1;
            let idx = prefix.partition_point(|&x| x < target);
            idx as i32
        })
        .collect()
}
