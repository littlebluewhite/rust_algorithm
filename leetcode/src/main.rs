use leetcode::question::q1391::has_valid_path;

fn main() {
    // Input: grid = [[2,4,3],[6,5,2]]
    // Output: true
    println!("{:?}", has_valid_path(vec![vec![2, 4, 3], vec![6, 5, 2]]));
    // Input: grid = [[4,1,3],[6,1,2]]
    // Output: true
    println!("{:?}", has_valid_path(vec![vec![4, 1, 3], vec![6, 1, 2]]));
    // Input: grid = [[4,3,3],[6,5,2]]
    // Output: false
    println!("{:?}", has_valid_path(vec![vec![4, 3, 3], vec![6, 5, 2]]));
}
