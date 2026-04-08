pub fn count_walls(lo:i32, hi:i32, walls: &Vec<i32>) -> i32{
    let lo_index = walls.partition_point(|&x| x < lo);
    let hi_index = walls.partition_point(|&x| x <= hi);
    (hi_index - lo_index) as i32
}

pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, mut walls: Vec<i32>) -> i32 {
    let n = robots.len();
    let mut robot_distance: Vec<(i32, i32)> = robots.into_iter().zip(distance).collect();
    robot_distance.sort_by_key(|&pos| pos.0);
    walls.sort_unstable();

    let mut right = vec![0;n];
    let mut left = vec![0;n];
    let mut num = vec![0;n];

    for i in 0..n{
        let (pos, dist) = robot_distance[i];
        let left_lo = if i ==0{
            pos-dist
        }else{
            (pos-dist).max(robot_distance[i-1].0+1)
        };
        left[i] = count_walls(left_lo, pos, &walls);

        let right_hi = if i == n-1{
            pos+dist
        }else{
            (pos+dist).min(robot_distance[i+1].0-1)
        };
        right[i] = count_walls(pos, right_hi, &walls);

        if i > 0{
            num[i] = count_walls(robot_distance[i-1].0, pos, &walls);
        }
    }

    let mut left_dp = left[0];
    let mut right_dp = right[0];
    for i in 1..n{
        let merged = (right[i-1] + left[i]).min(num[i]);
        let next_right_dp = (right_dp+right[i]).max(left_dp+right[i]);
        let next_left_dp = (left_dp+left[i]).max(right_dp-right[i-1] + merged);
        left_dp = next_left_dp;
        right_dp = next_right_dp;
    }
    left_dp.max(right_dp)
}