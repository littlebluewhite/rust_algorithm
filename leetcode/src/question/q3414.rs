type State = (i64, Vec<i32>);

pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let n = intervals.len();

    // ① 排序：原始索引依左端點排好
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&id| intervals[id][0]);
    let starts: Vec<i32> = order.iter().map(|&id| intervals[id][0]).collect();

    // ② 開表格：dp[i][k] = 從位置 i 往後看、最多選 k 個的最佳 State
    let mut dp: Vec<Vec<State>> = vec![vec![(0, vec![]); 5]; n + 1];

    // ③ 由後往前填：先找 j，再選或不選
    for i in (0..n).rev() {
        let id = order[i];
        let r = intervals[id][1];
        let w = intervals[id][2] as i64;
        let j = starts.partition_point(|&s| s <= r); // 第一個 start > r

        for k in 1..=4 {
            let skip = dp[i + 1][k].clone();

            let (neg, picks) = &dp[j][k - 1];
            let mut new_picks = picks.clone();
            new_picks.push(id as i32);
            new_picks.sort();
            let take = (neg - w, new_picks);

            dp[i][k] = skip.min(take);
        }
    }

    // ④ 回傳：最佳 State 的索引清單
    dp[0][4].1.clone()
}