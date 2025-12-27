use leetcode::question::q2402::most_booked;

fn main() {
    // Input: n = 2, meetings = [[0,10],[1,5],[2,7],[3,4]]
    // Output: 0
    println!("{:?}", most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]]));
    // Input: n = 3, meetings = [[1,20],[2,10],[3,5],[4,9],[6,8]]
    // Output: 1
    println!("{:?}", most_booked(3, vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]]));
    // Input: n = 4, meetings = [[18,19],[3,12],[17,19],[2,13],[7,10]]
    // Output: 0
    println!("{:?}", most_booked(4, vec![vec![18, 19], vec![3, 12], vec![17, 19], vec![2, 13], vec![7, 10]]));
    // Input: n = 2, meetings = [[43,44],[34,36],[11,47],[1,8],[30,33],[45,48],[23,41],[29,30]]
    // Output: 1
    println!("{:?}", most_booked(2, vec![vec![43, 44], vec![34, 36], vec![11, 47], vec![1, 8], vec![30, 33], vec![45, 48], vec![23, 41], vec![29, 30]]));
}
