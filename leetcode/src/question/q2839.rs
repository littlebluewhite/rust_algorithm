pub fn can_be_equal(s1: String, s2: String) -> bool {
    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();
    let n = b1.len();
    let mut diff = vec![vec![0;26];2];
    for i in 0..n {
        if i & 1 == 0{
            diff[0][(b1[i] - b'a') as usize] += 1;
            diff[0][(b2[i] - b'a') as usize] -= 1;
        }else{
            diff[1][(b1[i] - b'a') as usize] += 1;
            diff[1][(b2[i] - b'a') as usize] -= 1;
        }
    }
    diff.iter().all(|d| d.iter().all(|&v| v == 0))
}