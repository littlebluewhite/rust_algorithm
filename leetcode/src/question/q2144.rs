pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
    cost.sort();
    cost.reverse();
    cost.iter().enumerate().filter(|(i, _)| i % 3 != 2).map(|(_, c)| c).sum()
}