pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
    let max_value = *nums.iter().max().unwrap() as usize;
    let mut max_num = 1usize;
    while max_num <= max_value {
        max_num <<= 1;
    }
    let mut present = vec![false; max_num];
    let mut value: Vec<usize> = Vec::new();
    for num in nums{
        let num = num as usize;
        if !present[num]{
            present[num] = true;
            value.push(num);
        }
    }
    let mut seen = vec![false; max_num];
    let mut value_pair = Vec::new();
    for &a in &value{
        for &b in &value{
            let pair = a ^ b;
            if !seen[pair] {
                value_pair.push(pair);
                seen[pair] = true;
            }
        }
    }
    let mut ans = 0;
    seen = vec![false; max_num];
    for pair in value_pair{
        for &c in &value{
            let triple = pair ^ c;
            if !seen[triple] {
                seen[triple] = true;
                ans += 1;
            }
        }
    }
    ans
}