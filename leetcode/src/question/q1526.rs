pub fn min_number_operations(target: Vec<i32>) -> i32 {
    if target.is_empty(){
        return 0;
    }
    let mut result = target[0];
    for i in 1..target.len(){
        if target[i] > target[i-1]{
            result += target[i] - target[i-1];
        }
    }
    result
}