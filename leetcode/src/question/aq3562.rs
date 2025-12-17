pub fn max_profit(
    n: i32,
    present: Vec<i32>,
    future: Vec<i32>,
    hierarchy: Vec<Vec<i32>>,
    budget: i32,
) -> i32 {
    const NEG: i32 = -1_000_000_000;
    let b = budget as usize;
    let mut children = vec![Vec::new(); n as usize];
    for edge in hierarchy {
        let u = (edge[0] - 1) as usize;
        let v = (edge[1] - 1) as usize;
        children[u].push(v);
    }

    fn merge(base: Vec<i32>, child: &Vec<i32>, limit: usize) -> Vec<i32> {
        let mut res = vec![NEG; limit + 1];
        for i in 0..=limit {
            if base[i] == NEG {
                continue;
            }
            let remain = limit - i;
            for j in 0..=remain {
                let val = child[j];
                if val == NEG {
                    continue;
                }
                let new_cost = i + j;
                let new_val = base[i] + val;
                if new_val > res[new_cost] {
                    res[new_cost] = new_val;
                }
            }
        }
        res
    }

    fn build_option(
        base_cost: usize,
        base_profit: i32,
        child_states: &[(Vec<i32>, Vec<i32>)],
        use_discount_state: bool,
        limit: usize,
    ) -> Vec<i32> {
        let mut dp = vec![NEG; limit + 1];
        if base_cost <= limit {
            dp[base_cost] = base_profit;
        }
        for (c0, c1) in child_states {
            let child = if use_discount_state { c1 } else { c0 };
            dp = merge(dp, child, limit);
        }
        dp
    }

    fn dfs(
        u: usize,
        children: &Vec<Vec<usize>>,
        present: &Vec<i32>,
        future: &Vec<i32>,
        limit: usize,
    ) -> (Vec<i32>, Vec<i32>) {
        let mut sub = Vec::new();
        for &v in &children[u] {
            sub.push(dfs(v, children, present, future, limit));
        }

        let full_cost = present[u] as usize;
        let full_profit = future[u] - present[u];
        let disc_cost = (present[u] / 2) as usize;
        let disc_profit = future[u] - present[u] / 2;

        let opt_no_buy = build_option(0, 0, &sub, false, limit);
        let opt_buy_full = build_option(full_cost, full_profit, &sub, true, limit);
        let mut dp0 = vec![NEG; limit + 1];
        for i in 0..=limit {
            dp0[i] = opt_no_buy[i].max(opt_buy_full[i]);
        }

        let opt_buy_disc = build_option(disc_cost, disc_profit, &sub, true, limit);
        let mut dp1 = vec![NEG; limit + 1];
        for i in 0..=limit {
            dp1[i] = opt_no_buy[i].max(opt_buy_disc[i]);
        }

        (dp0, dp1)
    }

    let (root_dp, _) = dfs(0, &children, &present, &future, b);
    root_dp.into_iter().take(b + 1).max().unwrap_or(0)
}