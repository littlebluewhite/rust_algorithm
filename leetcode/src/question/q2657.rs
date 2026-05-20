pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let n = a.len();
    let mut ans = Vec::with_capacity(n);
    let mut count = 0;
    let mut seen: Vec<i32> = vec![0; n + 1];
    for i in 0..n {
        let x = a[i] as usize;
        seen[x] += 1;
        if seen[x] == 2 {
            count += 1;
            seen[x] = 0;
        }
        let x = b[i] as usize;
        seen[x] += 1;
        if seen[x] == 2 {
            count += 1;
            seen[x] = 0;
        }
        ans.push(count);
    }
    ans
}
