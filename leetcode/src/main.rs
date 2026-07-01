use leetcode::question::q2812::maximum_safeness_factor;

fn main() {
    // Input: grid = [[0,0,0,1],[0,0,0,0],[0,0,0,0],[1,0,0,0]]
    // Output: 2
    println!(
        "{:?}",
        maximum_safeness_factor(vec![
            vec![0, 0, 0, 1],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![1, 0, 0, 0]
        ])
    );
}
