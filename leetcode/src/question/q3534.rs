pub fn path_existence_queries(
    n: i32,
    nums: Vec<i32>,
    max_diff: i32,
    queries: Vec<Vec<i32>>,
) -> Vec<i32> {
    let n = n as usize;
    let mut new_nums = Vec::with_capacity(n);
    for (idx, &num) in nums.iter().enumerate() {
        new_nums.push((num, idx));
    }
    new_nums.sort_unstable();
    let mut idx_map = vec![0; n];
    let mut value = vec![0; n];
    for (i, &(v, idx)) in new_nums.iter().enumerate() {
        value[i] = v;
        idx_map[idx] = i;
    }
    let mut components = vec![0; n];
    let mut component_id = 0;
    let mut log = 1usize;
    while 1 << log < n {
        log += 1;
    }
    for i in 1..n {
        if value[i] - value[i - 1] > max_diff {
            component_id += 1;
        }
        components[i] = component_id;
    }
    let mut jump = vec![vec![0usize; n]; log];

    for i in 0..n {
        let limit = value[i] as i64 + max_diff as i64;
        let right = value.partition_point(|&x| x as i64 <= limit);
        jump[0][i] = right - 1;
    }
    for k in 1..log {
        for i in 0..n {
            jump[k][i] = jump[k - 1][jump[k - 1][i]];
        }
    }

    let mut ans = Vec::with_capacity(queries.len());
    for query in queries {
        let u = query[0] as usize;
        let v = query[1] as usize;
        if u == v {
            ans.push(0);
            continue;
        }
        let mut left = idx_map[u];
        let mut right = idx_map[v];
        if left > right {
            std::mem::swap(&mut left, &mut right);
        }
        if components[left] != components[right] {
            ans.push(-1);
            continue;
        }
        let mut step = 0;
        let mut cur = left;
        for k in (0..log).rev() {
            if jump[k][cur] < right {
                cur = jump[k][cur];
                step += 1 << k;
            }
        }
        ans.push(step + 1);
    }
    ans
}
