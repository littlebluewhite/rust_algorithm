pub fn min_swaps(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut zero_count: Vec<usize> = Vec::with_capacity(n);
    for i in 0..n{
        let mut zero = 0;
        for &j in grid[i].iter().rev(){
            if j == 0{
                zero += 1;
            }else{
                break;
            }
        }
        zero_count.push(zero);
    }
    let mut swap = 0;
    for i in 0..n{
        let need = n  -1 -i;
        let mut j = i;
        while j < n && zero_count[j] < need{
            j += 1;
        }
        if j == n{
            return -1;
        }
        while j > i{
            zero_count.swap(j, j-1);
            j -= 1;
            swap += 1;
        }
    }
    swap
}