use leetcode::question::q3756::sum_and_multiply;

fn main() {
    // Input: s = "10203004", queries = [[0,7],[1,3],[4,6]]
    // Output: [12340, 4, 9]
    println!(
        "{:?}",
        sum_and_multiply(
            "10203004".to_string(),
            vec![vec![0, 7], vec![1, 3], vec![4, 6]]
        )
    );
}
