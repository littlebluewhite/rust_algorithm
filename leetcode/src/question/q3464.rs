pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
    let k = k as usize;
    let side = side as i64;
    let perimeter = 4 * side;

    let mut arr: Vec<i64> = points
        .into_iter()
        .map(|p| to_perimeter(side, p[0] as i64, p[1] as i64))
        .collect();
    arr.sort_unstable();
    let mut ext: Vec<i64> = Vec::with_capacity(2 * arr.len());
    ext.extend(arr.iter().copied());
    ext.extend(arr.iter().map(|&x| perimeter + x));

    let mut lo = 1;
    let mut hi = side;
    while lo < hi {
        let mid = (lo + hi + 1) / 2;
        if can_pick(&ext, perimeter, k, mid) {
            lo = mid;
        } else {
            hi = mid - 1;
        }
    }
    lo as i32
}

fn to_perimeter(side: i64, x: i64, y: i64) -> i64 {
    if x == 0 {
        y
    } else if y == side {
        side + x
    } else if x == side {
        3 * side - y
    } else {
        4 * side - x
    }
}

fn can_pick(ext: &[i64], perimeter: i64, k: usize, dist: i64) -> bool {
    let n = ext.len()/2;
    for start in 0..n {
        let start_v = ext[start];
        let last_allowed = start_v + perimeter - dist;
        let mut idx = start;
        let mut ok = true;
        for _ in 1..k {
            let next = lower_bound_range(ext, idx + 1, start + n, ext[idx] + dist);
            if next == start + n || ext[next] > last_allowed {
                ok = false;
                break;
            }
            idx = next;
        }
        if ok {
            return true;
        }
    }
    false
}

fn lower_bound_range(ext: &[i64], mut left: usize, mut right: usize, target: i64) -> usize {
    while left < right {
        let mid = (left + right) / 2;
        if ext[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    left
}
