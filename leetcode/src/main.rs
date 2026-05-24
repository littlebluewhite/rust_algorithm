use leetcode::question::q1340::max_jumps;

fn main() {
    // Input: arr = [6,4,14,6,8,13,9,7,10,6,12], d = 2
    // Output: 4
    println!("{:?}", max_jumps(vec![6,4,14,6,8,13,9,7,10,6,12], 2));
    // Input: arr = [7,6,5,4,3,2,1], d = 1
    // Output: 7
    println!("{:?}", max_jumps(vec![7,6,5,4,3,2,1], 1));
}
