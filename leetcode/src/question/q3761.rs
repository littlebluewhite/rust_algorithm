use std::collections::HashMap;

pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut ans = i32::MAX;
    for j in 0..n {
        if let Some(&i) = map.get(&nums[j]) {
            ans = ans.min((j - i) as i32);
        }
        map.insert(reverse(nums[j]), j);
    }
    if ans == i32::MAX { -1 } else { ans }
}

fn reverse(mut a: i32) -> i32 {
    let mut result = 0;
    while a > 0 {
        result = result * 10 + a % 10;
        a /= 10;
    }
    result
}
