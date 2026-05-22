pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
    let n = grid.len();
    debug_assert!(n > 0);
    debug_assert!(grid.iter().all(|row| row.len() == n));

    let mut pref = vec![vec![0i64; n + 1]; n];
    for col in 0..n {
        for row in 0..n {
            pref[col][row + 1] = pref[col][row] + grid[row][col] as i64;
        }
    }

    const NEG: i64 = i64::MIN / 4;
    let zero_pref = vec![0i64; n + 1];
    let mut prev_col_pref = zero_pref.clone();

    let mut prev = vec![vec![NEG; n + 1]; n];
    prev[0][0] = 0;
    for col in 0..=n {
        let curr_col_pref = if col < n { &pref[col] } else { &zero_pref };

        let mut suffix = vec![vec![NEG; n + 1]; n + 1];
        let mut prefix_adj = vec![vec![0i64; n + 1]; n + 1];

        for h_prev in 0..=n {
            let mut best_suffix = NEG;
            for k in (0..=n).rev() {
                best_suffix = best_suffix.max(prev[h_prev][k]);
                suffix[h_prev][k] = best_suffix;
            }

            let mut best_prefix = NEG;
            for k in 0..=n {
                if prev[h_prev][k] > NEG / 2 {
                    let already_counted = if k > h_prev {
                        prev_col_pref[k] - prev_col_pref[h_prev]
                    } else {
                        0
                    };
                    best_prefix = best_prefix.max(prev[h_prev][k] - already_counted);
                }
                prefix_adj[h_prev][k] = best_prefix;
            }
        }

        let mut curr = vec![vec![NEG; n + 1]; n + 1];
        let max_h_curr = if col == n { 0 } else { n };

        for h_curr in 0..=max_h_curr {
            for h_prev in 0..=n {
                if h_curr <= h_prev {
                    let base = suffix[h_prev][0];
                    if base > NEG / 2 {
                        curr[h_curr][h_prev] = base + curr_col_pref[h_prev] - curr_col_pref[h_curr];
                    }
                } else {
                    let mut best = NEG;

                    let no_extra = suffix[h_prev][h_curr];
                    if no_extra > NEG / 2 {
                        best = best.max(no_extra);
                    }
                    let adjusted = prefix_adj[h_prev][h_curr];
                    if adjusted > NEG / 2 {
                        best = best.max(
                            adjusted + prev_col_pref[h_curr] - prev_col_pref[h_prev],
                        );
                    }

                    curr[h_curr][h_prev] = best;
                }
            }
        }
        prev = curr;
        prev_col_pref = curr_col_pref.to_vec();
    }
    prev[0].iter().copied().max().unwrap()
}
