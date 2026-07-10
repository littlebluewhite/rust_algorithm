pub fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
    let n = n as usize;
    let mut ans: Vec<bool> = Vec::with_capacity(queries.len());
    let mut component: Vec<i32> = vec![0; n];
    let mut component_id = 0;
    for i in 1..n{
        if nums[i] - nums[i - 1] > max_diff{
            component_id += 1;
        }
        component[i] = component_id;
    }
    for query in queries{
        let (a, b) = (query[0] as usize, query[1] as usize);
        ans.push(component[a] == component[b]);
    }
    ans
}