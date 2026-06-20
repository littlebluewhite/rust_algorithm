use leetcode::question::q1840::max_building;

fn main() {
    // Input: n = 10, restrictions = [[8,5],[9,0],[6,2],[4,0],[3,2],[10,0],[5,3],[7,3],[2,4]]
    // Output: 2
    println!("{:?}", max_building(10, vec![vec![8, 5], vec![9, 0], vec![6, 2], vec![4, 0], vec![3, 2], vec![10, 0], vec![5, 3], vec![7, 3], vec![2, 4]]));
}
