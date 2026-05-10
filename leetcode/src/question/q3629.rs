use std::collections::VecDeque;

pub fn min_jumps(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return 0;
    }
    let max_value = nums.iter().map(|&x| x as usize).max().unwrap();
    let prime_map = build_is_prime(max_value);

    let mut position: Vec<Vec<usize>> = vec![Vec::new(); max_value + 1];
    for (i, &v) in nums.iter().enumerate() {
        position[v as usize].push(i);
    }

    let mut dist = vec![-1; n];
    let mut prime_used = vec![false; max_value + 1];
    let mut q: VecDeque<usize> = VecDeque::new();
    dist[0] = 0;
    q.push_back(0usize);
    while let Some(i) = q.pop_front() {
        if i == n - 1 {
            return dist[i];
        }
        let next = dist[i] + 1;
        if i + 1 < n && dist[i + 1] == -1 {
            dist[i + 1] = next;
            q.push_back(i + 1);
        }
        if i > 0 && dist[i - 1] == -1 {
            dist[i - 1] = next;
            q.push_back(i - 1);
        }
        let p = nums[i] as usize;
        if prime_map[p] && !prime_used[p] {
            prime_used[p] = true;
            let mut multiple = p;
            while multiple <= max_value {
                for &index in position[multiple].iter() {
                    if dist[index] == -1 {
                        dist[index] = next;
                        q.push_back(index);
                    }
                }
                multiple += p;
            }
        }
    }
    dist[n - 1]
}

fn build_is_prime(limit: usize) -> Vec<bool> {
    let mut res = vec![true; limit + 1];
    res[0] = false;
    if limit >= 1 {
        res[1] = false;
    }
    let mut i = 2usize;
    while i * i <= limit {
        if res[i] {
            let mut multiple = i * i;
            while multiple <= limit {
                res[multiple] = false;
                multiple += i;
            }
        }
        i += 1
    }
    res
}
