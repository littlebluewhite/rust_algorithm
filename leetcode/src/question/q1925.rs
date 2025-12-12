pub fn count_triples(n: i32) -> i32 {
    let mut ans = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let big = i * i + j * j;
            if big > n * n {
                break;
            }
            let big_root = (big as f64).sqrt() as i32;
            println!("{} {} {} {}", i, j, big_root, big_root * big_root);
            if big_root * big_root == i * i + j * j && big_root <= n {
                ans += 2;
            }
        }
    }
    ans
}
