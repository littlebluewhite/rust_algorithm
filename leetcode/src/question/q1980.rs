pub fn find_different_binary_string(nums: Vec<String>) -> String {
    let n = nums.len();
    let mut ans: Vec<u8> = Vec::with_capacity(n);
    for i in 0..n{
        let ch = if nums[i].as_bytes()[i] == b'0'{b'1'}else{b'0'};
        ans.push(ch)
    }
    String::from_utf8(ans).unwrap()
}