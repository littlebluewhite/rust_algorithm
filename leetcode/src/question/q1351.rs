pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0i32;
    for row in grid{
        for val in row{
            if val < 0{
                ans += 1
            }
        }
    }
    ans
}