use leetcode::question::q3651::min_cost;

fn main() {
    // Input: grid = [[1,3,3],[2,5,4],[4,3,5]], k = 2
    // Output: 7
    println!(
        "{}",
        min_cost(vec![vec![1, 3, 3], vec![2, 5, 4], vec![4, 3, 5]], 2)
    );

    // Input: grid = [[1,2],[2,3],[3,4]], k = 1
    // Output: 9
    println!("{}", min_cost(vec![vec![1, 2], vec![2, 3], vec![3, 4]], 1));
}
