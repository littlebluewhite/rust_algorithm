use leetcode::question::q3559::assign_edge_weights;

fn main() {
    // Input: edges = [[1,2]], queries = [[1,1],[1,2]]
    // Output: [0,1]
    println!("{:?}", assign_edge_weights(vec![vec![1, 2]], vec![vec![1, 1], vec![1, 2]]));
}
