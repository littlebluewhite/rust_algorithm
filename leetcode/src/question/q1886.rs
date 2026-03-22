pub fn find_rotation(mut mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
    let n = mat.len();
    let mut i = 0;
    while i<4 {
        if is_match(&mat, &target){
            return true;
        }
        let mut new_mat = vec![vec![0;n];n];
        for r in 0..n{
            for c in 0..n{
                new_mat[c][n-1-r] = mat[r][c];
            }
        }
        mat = new_mat;
        i+=1;
    }
    false
}

fn is_match(mat: &Vec<Vec<i32>>, target: &Vec<Vec<i32>>) -> bool{
    let n = mat.len();
    for r in 0..n{
        for c in 0..n{
            if mat[r][c] != target[r][c]{
                return false
            }
        }
    }
    true
}
// 0,0 -> 0,2
// 0,1 -> 1,2
// 0,2 -> 2,2
// 1,0 -> 0,1
// 1,1 -> 1,1
// 1,2 -> 2,1
// 2,0 -> 0,0
// 2,1 -> 1,0
// 2,2 -> 2,0
