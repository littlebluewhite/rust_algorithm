pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
    let n = nums.len();
    let mut value: Vec<(i32, usize)> = nums.into_iter().enumerate().map(|(i, v)| (v, i)).collect();
    value.sort_unstable();
    let mut ans = vec![0; n];
    let mut start = 0usize;
    let mut end = start+1;
    while start < n{
        let mut end = start + 1;
        while end < n && value[end].0 - value[end-1].0 <= limit {
            end += 1;
        }
        let mut indices: Vec<usize> = value[start..end].iter().map(|&(_, i)| i).collect();
        indices.sort_unstable();
        for (offset, &index) in indices.iter().enumerate() {
            ans[index] = value[start + offset].0;
        }
        start = end;
    }
    ans
}