pub fn max_depth_after_split(seq: String) -> Vec<i32> {
    let b = seq.as_bytes();
    let n = b.len();
    let mut res = Vec::with_capacity(seq.len());
    let mut depth = 0;
    for i in 0..n{
        if b[i] == b'(' {
            res.push(depth&1);
            depth += 1;
        }else{
            depth -= 1;
            res.push(depth&1);
        }
    }
    res
}