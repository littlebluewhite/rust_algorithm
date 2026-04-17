use leetcode::question::q3488::solve_queries;

fn main() {
    // Input: nums = [1,3,1,4,1,3,2], queries = [0,3,5]
    // Output: [2,-1,3]
    println!(
        "{:?}",
        solve_queries(vec![1, 3, 1, 4, 1, 3, 2], vec![0, 3, 5])
    );
}
