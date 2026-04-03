pub fn survived_robots_healths(positions: Vec<i32>, mut healths: Vec<i32>, directions: String) -> Vec<i32> {
    let n = positions.len();
    let directions = directions.as_bytes();
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_by_key(|&i| positions[i]);
    let mut stack: Vec<usize> = Vec::new();
    for i in order{
        if directions[i] == b'R' {
            stack.push(i);
            continue;
        }
        while healths[i] > 0{
            if let Some(j) = stack.pop(){
                if healths[i] > healths[j]{
                    healths[i] -= 1;
                    healths[j] = 0;
                }else if healths[i] < healths[j]{
                    healths[j] -= 1;
                    healths[i] = 0;
                    stack.push(j);
                    break;
                }else{
                    healths[i] = 0;
                    healths[j] = 0;
                }
            }else{
                break;
            }
        }
    }
    healths.into_iter().filter(|&h| h > 0).collect()
}