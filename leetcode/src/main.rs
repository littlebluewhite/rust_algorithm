use leetcode::question::q1292::max_side_length;

fn main(){
    // Input: mat = [[1,1,3,2,4,3,2],[1,1,3,2,4,3,2],[1,1,3,2,4,3,2]], threshold = 4
    // Output: 2
    println!("{}", max_side_length(vec![vec![1,1,3,2,4,3,2],vec![1,1,3,2,4,3,2],vec![1,1,3,2,4,3,2]], 4));

    // Input: mat = [[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2],[2,2,2,2,2]], threshold = 1
    // Output: 0
    println!("{}", max_side_length(vec![vec![2,2,2,2,2],vec![2,2,2,2,2],vec![2,2,2,2,2],vec![2,2,2,2,2],vec![2,2,2,2,2]], 1));
}
