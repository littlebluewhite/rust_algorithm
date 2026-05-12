pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
    let side = side as i64;
    let perimeter = 4 * side;
    let k = k as usize;

    let mut arr: Vec<i64> = points
        .into_iter()
        .map(|p| to_perimeter(side, p[0] as i64, p[1] as i64))
        .collect();
    arr.sort_unstable();

    let mut ext: Vec<i64> = Vec::with_capacity(arr.len() * 2);
    ext.extend(arr.iter().copied());
    ext.extend(arr.iter().map(|&x| x + perimeter));

    let mut lo = 1i64;
    let mut hi = side;
    while lo < hi {
        let mid = (hi + lo + 1) / 2;
        if can_pick(&arr, &ext, perimeter, k, mid) {
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
        side * 3 - y
    } else {
        side * 4 - x
    }
}

fn can_pick(arr: &[i64], ext: &[i64], perimeter: i64, k: usize, dist: i64) -> bool {
    let n = arr.len();

    for start in 0..n {
        let first = ext[start];
        let last_allowed = first + perimeter - dist;
        let mut idx = start;
        let mut ok = true;

        for _ in 1..k {
            let next = lower_bound_range(ext, idx + 1, start + n, ext[idx] + dist);
            if next == start + n || ext[next] > last_allowed {
                ok = false;
                break;
            }

            idx = next
        }
        if ok {
            return true;
        }
    }
    false
}

fn lower_bound_range(arr: &[i64], mut left: usize, mut right: usize, target: i64) -> usize {
    while left < right {
        let mid = (left + right) / 2;
        if arr[mid] >= target {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}
