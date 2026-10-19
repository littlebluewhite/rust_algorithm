const MOD: i64 = 1000000007;

pub fn num_of_ways(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut choose = vec![vec![0; n + 1]; n + 1];
    choose[0][0] = 1;
    for a in 1..=n {
        choose[a][0] = 1;
        choose[a][a] = 1;
        for b in 1..a {
            choose[a][b] = (choose[a - 1][b - 1] + choose[a - 1][b]) % MOD;
        }
    }
    let mut left_child: Vec<Option<usize>> = vec![None; n];
    let mut right_child: Vec<Option<usize>> = vec![None; n];
    for i in 1..n {
        let mut u = 0;
        loop {
            let child = if nums[i] < nums[u] {
                &mut left_child[u]
            } else {
                &mut right_child[u]
            };
            match *child {
                Some(v) => u = v,
                None => {
                    *child = Some(i);
                    break;
                }
            }
        }
    }
    let mut size = vec![1; n];
    let mut ways = vec![1; n];
    for u in (0..n).rev() {
        let (left_size, left_ways) = match left_child[u] {
            Some(i) => (size[i], ways[i]),
            None => (0, 1),
        };
        let (right_size, right_ways) = match right_child[u] {
            Some(i) => (size[i], ways[i]),
            None => (0, 1),
        };
        size[u] = left_size + right_size + 1;
        ways[u] =
            ((choose[left_size + right_size][left_size] * left_ways) % MOD * right_ways) % MOD;
    }
    ((ways[0] - 1 + MOD) % MOD) as i32
}
