use leetcode::question::q1970::latest_day_to_cross;

fn main(){
    // Input: row = 2, col = 2, cells = [[1,1],[2,1],[1,2],[2,2]]
    // Output: 2
    println!(
        "{:?}",
        latest_day_to_cross(2, 2, vec![vec![1,1],vec![2,1],vec![1,2],vec![2,2]])
    );

    // Input: row = 2, col = 2, cells = [[1,1],[1,2],[2,1],[2,2]]
    // Output: 1
    println!(
        "{:?}",
        latest_day_to_cross(2, 2, vec![vec![1,1],vec![1,2],vec![2,1],vec![2,2]])
    );

    // Input: row = 3, col = 3, cells = [[1,2],[2,1],[3,3],[2,2],[1,1],[1,3],[2,3],[3,2],[3,1]]
    // Output: 3
    println!(
        "{:?}",
        latest_day_to_cross(3, 3, vec![vec![1,2],vec![2,1],vec![3,3],vec![2,2],vec![1,1],vec![1,3],vec![2,3],vec![3,2],vec![3,1]])
    );
}
