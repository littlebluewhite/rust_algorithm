use leetcode::question::q3286::find_safe_walk;

fn main() {
    // Input: grid = [[0,1,0,0,0],[0,1,0,1,0],[0,0,0,1,0]], health = 1
    // Output: true
    println!(
        "{:?}",
        find_safe_walk(
            vec![
                vec![0, 1, 0, 0, 0],
                vec![0, 1, 0, 1, 0],
                vec![0, 0, 0, 1, 0]
            ],
            1
        )
    );
}
