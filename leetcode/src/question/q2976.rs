
pub fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {
    let source = source.as_bytes();
    let target = target.as_bytes();
    const INF:i64 = i64::MAX/4;
    let mut dist = vec![vec![INF; 26]; 26];
    for i in 0..26{
        dist[i][i] = 0;
    }
    
    for i in 0..original.len(){
        let u = original[i] as usize - 'a' as usize;
        let v = changed[i] as usize - 'a' as usize;
        dist[u][v] = dist[u][v].min(cost[i] as i64);
    }
    
    for k in 0..26{
        for i in 0..26{
            if dist[i][k] == INF{
                continue;
            }
            for j in 0..26{
                if dist[k][j] == INF{
                    continue;
                }
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }
    let mut ans = 0i64;
    for i in 0..source.len(){
        if source[i] == target[i]{
            continue;
        }
        let u = source[i] as usize - 'a' as usize;
        let v = target[i] as usize - 'a' as usize;
        let c = dist[u][v];
        if c == INF{
            return -1
        }else{
            ans += c;
        }
    }
    ans
}