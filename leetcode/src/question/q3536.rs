pub fn max_product(mut n: i32) -> i32 {
    let mut vec_n = Vec::new();
    while n > 0 {
        vec_n.push(n % 10);
        n /= 10;
    }
    let mut max = 0;
    for i in 0..vec_n.len()-1 {
        for j in i+1..vec_n.len() {
            max = max.max(vec_n[i]*vec_n[j]);
        }
    }
    max
}