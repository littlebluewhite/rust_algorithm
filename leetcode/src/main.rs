use leetcode::question::q2092::find_all_people;

fn main() {
    // Input: n = 6, meetings = [[1,2,5],[2,3,8],[1,5,10]], firstPerson = 1
    println!("{:?}", find_all_people(6, vec![vec![1,2,5],vec![2,3,8],vec![1,5,10]], 1));
}
