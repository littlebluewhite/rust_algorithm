const INF: i64 = 1 << 60;
pub fn minimum_total_distance(mut robot: Vec<i32>, mut factory: Vec<Vec<i32>>) -> i64 {
    robot.sort_unstable();
    factory.sort_by_key(|x| x[0]);
    let r = robot.len();
    let f = factory.len();
    let factory: Vec<(i32, usize)> = factory.iter().map(|x| (x[0], x[1] as usize)).collect();
    let mut memo = vec![vec![None; f + 1]; r + 1];
    fn dfs(
        r: usize,
        f: usize,
        robot: &Vec<i32>,
        factory: &Vec<(i32,usize)>,
        memo: &mut [Vec<Option<i64>>],
    ) -> i64 {
        if r == robot.len(){
            return 0;
        }
        if f == factory.len(){
            return INF;
        }
        if let Some(v) = memo[r][f] {
            return v;
        }
        let(f_dist, f_limit) = factory[f];
        let mut ans = INF;
        ans = ans.min(dfs(r, f+1, robot, factory, memo));
        let mut dist = 0;
        for k in 1..=f_limit {
            if r + k > robot.len() {
                break;
            }
            let fix_robot_position = robot[r+k-1];
            dist += (fix_robot_position - f_dist).abs() as i64;
            ans = ans.min(dist + dfs(r+k, f+1, robot, factory, memo))
        }
        memo[r][f] = Some(ans);
        ans
    }
    dfs(0, 0, &robot, &factory, &mut memo)
}
