use std::collections::{HashMap};

pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let k = k as usize;
    let x = x as usize;
    let mut res = Vec::with_capacity(nums.len()-k+1);
    for i in 0..=nums.len() - k {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for &v in &nums[i..i + k]{
            *freq.entry(v).or_insert(0)+=1;
        }
        let mut items: Vec<(i32, i32)> = freq.into_iter().collect();
        items.sort_by(|a, b| {
            match b.1.cmp(&a.1){
                std::cmp::Ordering::Equal => b.0.cmp(&a.0),
                other => other,
            }
        });
        let mut sum = 0i32;
        for (i, (val, cnt)) in items.into_iter().enumerate() {
            if i >= x{
                break;
            }
            sum += val * cnt;
        }
        res.push(sum)
    }
    res
}
