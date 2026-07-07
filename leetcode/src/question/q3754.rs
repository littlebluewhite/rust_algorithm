pub fn sum_and_multiply(mut n: i32) -> i64 {
    let mut x_vec: Vec<i32> = Vec::new();
    let mut x = 0;
    let mut sum = 0;
    while n > 0{
        if n%10 == 0{
        }else{
            x_vec.push(n%10);
            sum += n%10;

        }
        n /= 10;
    }
    x_vec.reverse();
    for &i in &x_vec{
        x = x * 10+i;
    }

    x as i64 * sum as i64
}