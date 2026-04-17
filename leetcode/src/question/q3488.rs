use std::collections::HashMap;

fn find_distance(a: usize, b: usize, n: usize) -> usize{
    let diff = a.abs_diff(b);
    diff.min(n-diff)
}

pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut map: HashMap<i32, Vec<usize>> = HashMap::new();
    for i in 0..nums.len() {
        map.entry(nums[i]).or_default().push(i);
    }
    let mut best = vec![-1; n];
    for pos in map.values(){
        let k = pos.len();
        if k == 1{
            continue;
        }
        for index in 0..k{
            let curr = pos[index];
            let prev = pos[(index+k-1)%k];
            let next = pos[(index+1)%k];

            let d1 = find_distance(curr, prev, n);
            let d2 = find_distance(curr, next, n);
            best[curr] = d1.min(d2) as i32;
        }
    }
    queries.into_iter().map(|x| best[x as usize]).collect()
}