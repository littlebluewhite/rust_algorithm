pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let n = strs.len();
    let m = strs[0].len();
    let mut deletions = 0;
    let str_b = strs.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
    let mut sorted = vec![false; n];
    for col in 0..m{
        let mut bad = false;
        for i in 1..n{
            if !sorted[i] && str_b[i][col] < str_b[i - 1][col] {
                bad = true;
                break;
            }
        }
        if bad{
            deletions += 1;
            continue;
        }
        for i in 1..n{
            if !sorted[i] && str_b[i][col] > str_b[i - 1][col] {
                sorted[i] = true;
            }
        }
    }
    deletions
}