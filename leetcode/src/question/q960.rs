pub fn min_deletion_size(strs: Vec<String>) -> i32 {
    let n = strs.len();
    let mut strs_b: Vec<Vec<u8>> = strs.iter().map(|s| s.as_bytes().to_vec()).collect();
    let mut deletions = Vec::new();
    let mut ans = 0;
    for i in 0..n{
        strs_b = strs_b.iter().map(|s| s.iter().enumerate().filter(|(i, _)| {
            !deletions.contains(i)
        }).map(|(_, &v)| v).collect::<Vec<u8>>()).collect();
        println!("{:?}", strs_b);
        ans += deletions.len() as i32;
        deletions.clear();
        let mut pre = strs_b[i][0];
        for col in 1..strs_b[i].len(){
            if pre > strs_b[i][col]{
                deletions.push(col);
            }else{
                pre = strs_b[i][col];
            }
        }
    }
    ans+deletions.len() as i32
}