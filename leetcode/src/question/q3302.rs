pub fn valid_sequence(word1: String, word2: String) -> Vec<i32> {
    let source = word1.as_bytes();
    let target = word2.as_bytes();
    let source_len = source.len();
    let target_len = target.len();
    let mut last = vec![-1i32;target_len+1];
    last[target_len] = source_len as i32;
    let mut target_idx = target_len;
    for i in (0..source_len).rev(){
        if target_idx > 0 && target[target_idx-1] == source[i] {
            target_idx -= 1;
            last[target_idx] = i as i32;
        }
    }
    let mut ans = Vec::new();
    let mut mismatch_used = false;
    target_idx = 0;
    for i in 0..source_len{
        if target_idx == target_len{
            break;
        }
        let matched = source[i] == target[target_idx];
        let affordable = !mismatch_used && (i as i32+1) <= last[target_idx+1];
        if matched || affordable{
            if !matched {
                mismatch_used = true;
            }
            target_idx += 1;
            ans.push(i as i32);
        }
    }
    if target_idx == target_len{
        ans
    }else{
        vec![]
    }
}