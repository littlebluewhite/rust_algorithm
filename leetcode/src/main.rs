use leetcode::question::q3453::separate_squares;

fn main(){
    // Input: squares = [[0,0,1],[2,2,1]]
    // Output: 1.00000
    println!("{:?}", separate_squares(vec![vec![0,0,1],vec![2,2,1]]));

    // Input: squares = [[0,0,2],[1,1,1]]
    // Output: 1.16667
    println!("{:?}", separate_squares(vec![vec![0,0,2],vec![1,1,1]]));
}
