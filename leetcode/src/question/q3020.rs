use std::collections::HashMap;

pub fn maximum_length(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i64, i32> = HashMap::new();
    let mut max = 0i64;
    for &num in &nums{
        let num = num as i64;
        map.entry(num).and_modify(|v| *v += 1).or_insert(1);
        max = max.max(num);
    }
    let mut ans = 0;
    if let Some(&ones) = map.get(&1) {
        let base_ones = if ones % 2 == 1 { ones } else { ones - 1 };
        ans = ans.max(base_ones)
    }
    let keys: Vec<i64> = map.keys().copied().collect();
    for key in keys {
        if key == 1 {
            continue;
        }
        let mut value = key;
        let mut pairs = 0;
        while let Some(&count) = map.get(&value) {
            ans = ans.max(pairs * 2 + 1);
            if count < 2 || value > max/value{
                break;
            }
            let next = value * value;
            value = next;
            pairs += 1;
        }
    }
    ans
}