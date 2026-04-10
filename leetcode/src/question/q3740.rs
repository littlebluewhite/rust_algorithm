pub fn minimum_distance(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut count: Vec<Vec<usize>> = vec![vec![]; n+1];
    for (i, &num) in nums.iter().enumerate() {
        count[num as usize].push(i);
    }
    let mut ans = usize::MAX;
    for positions in count{
        if positions.len() < 3 {
            continue;
        }
        for i in 0..positions.len() - 2 {
            let distance = 2 * (positions[i+2] - positions[i]);
            ans = ans.min(distance);
        }
    }
    if ans == usize::MAX {
        return -1;
    }
    ans as i32
}