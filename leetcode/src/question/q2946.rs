pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
    let m = mat[0].len();
    let shift = k as usize % m;
    if shift == 0 {
        return true;
    }
    for row in mat.iter(){
        for j in 0..m{
            if row[j] != row[(j + shift) % m] {
                return false;
            }
        }
    }
    true
}