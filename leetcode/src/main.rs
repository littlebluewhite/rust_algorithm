use leetcode::question::q840::num_magic_squares_inside;

fn main(){
    // Input: grid = [[4,3,8,4],[9,5,1,9],[2,7,6,2]]
    // Output: 1
    println!(
        "{:?}",
        num_magic_squares_inside(vec![vec![4,3,8,4],vec![9,5,1,9],vec![2,7,6,2]])
    );
}
