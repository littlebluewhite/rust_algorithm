use std::collections::HashMap;

pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut l = 0usize;
    let mut count = 0;
    for r in 0..nums.len(){
        let num = nums[r];
        map.entry(num).and_modify(|v| *v += 1).or_insert(1);
        while map.get(&num) > Some(&k) {
            let remove_count = map.get_mut(&nums[l]).unwrap();
            *remove_count -= 1;
            if *remove_count == 0{
                map.remove(&nums[l]);
            }
            l += 1;
        }
        count = count.max(r - l + 1);
    }
    count as i32
}