use leetcode::question::aq2054::max_two_events;

fn main() {
    // Input: events = [[1,3,2],[4,5,2],[2,4,3]]
    // Output: 4
    println!(
        "{:?}",
        max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]])
    );
    // Input: events = [[1,3,2],[4,5,2],[1,5,5]]
    // Output: 5
    println!(
        "{:?}",
        max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]])
    );
    // Input: events = [[1,5,3],[1,5,1],[6,6,5]]
    // Output: 8
    println!(
        "{:?}",
        max_two_events(vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]])
    );
    // Input: events = [[10,83,53],[63,87,45],[97,100,32],[51,61,16]]
    // Output: 85
    println!(
        "{:?}",
        max_two_events(vec![vec![10, 83, 53], vec![63, 87, 45], vec![97, 100, 32], vec![51, 61, 16]])
    );
}
