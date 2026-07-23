pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
    let n: usize = nums.len();
    match n {
        1 => 1,
        2 => 2,
        _ => 1 << (1 + n.ilog2())
    }
}