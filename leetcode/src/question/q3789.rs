pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
    let (cost1, cost2, cost_both, need1, need2) = (
        cost1 as i64,
        cost2 as i64,
        cost_both as i64,
        need1 as i64,
        need2 as i64,
    );
    let (hi, lo) = (need1.max(need2), need1.min(need2));
    let cost = |both: i64|->i64{
        both*cost_both + (need1-both).max(0)*cost1+(need2-both).max(0)*cost2
    };
    cost(0).min(cost(hi)).min(cost(lo))
}
