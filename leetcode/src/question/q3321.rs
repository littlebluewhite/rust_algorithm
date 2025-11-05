use std::collections::HashMap;

pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
    let k = k as usize;
    let x = x as usize;
    let mut res = Vec::with_capacity(nums.len()-k+1);
    for i in 0..=nums.len()-k{
        let mut h:HashMap<i32, i32> = HashMap::new();
        for &n in &nums[i..i+k]{
            *h.entry(n).or_insert(0)+=1;
        }
        let mut items: Vec<(i32,i32)> = h.into_iter().collect();
        items.sort_by(|a, b| {
            match b.1.cmp(&a.1){
                std::cmp::Ordering::Equal => b.0.cmp(&a.0),
                ord => ord,
            }
        });
        let mut sum = 0i64;
        let mut i = 0;
        while i < items.len(){
            if i >= x{
                break;
            }
            sum += items[i].0 as i64 * items[i].1 as i64;
            i += 1;
        }
        res.push(sum);
    }
    res
}