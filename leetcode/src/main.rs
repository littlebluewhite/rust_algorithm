use leetcode::question::q1386::max_number_of_families;

fn main() {
    // Input: n = 3, reservedSeats = [[1,2],[1,3],[1,8],[2,6],[3,1],[3,10]]
    // Output: 4
    println!("{:?}", max_number_of_families(3, vec![vec![1,2], vec![1,3], vec![1,8], vec![2,6], vec![3,1], vec![3,10]]));
}
