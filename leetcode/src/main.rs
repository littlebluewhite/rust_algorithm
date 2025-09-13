use leetcode::question::q3541::max_freq_sum;

fn main() {
    let s1 = "successes".to_string();
    println!("{}", max_freq_sum(s1)); // 輸出: 6

    let s2 = "aeieiaia".to_string();
    println!("{}", max_freq_sum(s2)); // 輸出: 3

}
