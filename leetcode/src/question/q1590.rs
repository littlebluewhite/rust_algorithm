use std::collections::HashMap;

pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let p = p as i64;
    let mut sum = 0i64;
    for &n in &nums {
        sum += n as i64;
    }
    let need = sum % p;
    if need == 0 {
        return 0;
    }
    let mut ans = (nums.len() + 1) as i32;
    let mut map: HashMap<i64, i32> = HashMap::new();
    map.insert(0, -1);
    let mut prefix = 0i64;
    for (i, &n) in nums.iter().enumerate() {
        prefix = (n as i64 + prefix) % p;
        let target = (prefix - need + p) % p;
        if let Some(&j) = map.get(&target) {
            let len = i as i32 - j;
            if len < ans {
                ans = len
            }
        }
        map.insert(prefix, i as i32);
    }
    if ans >= nums.len() as i32 {
        -1
    } else {
        ans
    }
}
