
pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut db = triangle.last().unwrap().clone();
    for i in (0..triangle.len()-1).rev(){
        for j in 0..triangle[i].len(){
            db[j] = triangle[i][j] + db[j].min(db[j+1])
        }
    }
    db[0]
}