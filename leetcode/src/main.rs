use leetcode::question::q3093::string_indices;

fn main() {
    // Input: wordsContainer = ["abcde","abcde"], wordsQuery = ["abcde","bcde","cde","de","e"]
    // Output: [1,1,1,1,1]
    println!("{:?}", string_indices(vec!["abcde".to_string(), "abcde".to_string()], vec!["abcde".to_string(), "bcde".to_string(), "cde".to_string(), "de".to_string(), "e".to_string()]));
}
