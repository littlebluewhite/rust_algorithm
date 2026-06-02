use leetcode::question::q3633::earliest_finish_time;

fn main() {
    // Input: landStartTime = [2,8], landDuration = [4,1], waterStartTime = [6], waterDuration = [3]
    // Output: 9
    println!("{:?}", earliest_finish_time(vec![2, 8], vec![4, 1], vec![6], vec![3]));
}
