pub fn max_distance(colors: Vec<i32>) -> i32 {
    let mut ans = 0;
    let n = colors.len();
    let first = colors[0];
    let last = colors[n-1];
    for i in 0..n{
        if colors[i] != first{
            ans = ans.max(i);
        }
        if colors[i] != last{
            ans = ans.max(n-i-1);
        }
    }
    ans as i32
}