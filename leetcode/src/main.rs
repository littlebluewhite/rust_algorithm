use leetcode::question::q2872::max_k_divisible_components;
fn main() {
    // edges =
    //     [[0,2],[1,2],[1,3],[2,4]]
    // values =
    //     [1,8,1,4,4]
    println!("{}", max_k_divisible_components(5, vec![vec![0,2],vec![1,2],vec![1,3],vec![2,4]], vec![1,8,1,4,4], 4));
}
