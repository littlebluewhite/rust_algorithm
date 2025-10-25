pub fn next_beautiful_number(n: i32) -> i32 {
    // 產生所有 ≤ 7 位的數值平衡數（包含所有排列）
    let mut cand = Vec::new();
    let mut cnt = [0i32; 10];

    // 枚舉要使用哪些數字（1..7）。若使用 d，則必須用 d 次。
    for mask in 1usize..(1usize << 7) {
        println!("mask: {}", mask);
        let mut len = 0;
        for d in 1..=7 {
            if (mask >> (d - 1)) & 1 == 1 {
                len += d;              // 總長度要加 d
                cnt[d as usize] = d as i32;
            } else {
                cnt[d as usize] = 0;
            }
        }
        if len > 7 {                   // 超過 7 位就不可能 ≤ 1e6
            continue;
        }
        // 回溯出所有排列
        permute(&mut cnt, len as i32, 0i64, 0, &mut cand);
    }

    cand.sort_unstable();
    cand.dedup();
    println!("cand: {:?}", cand);

    // 回傳第一個嚴格大於 n 的
    for &x in &cand {
        if x > n {
            return x;
        }
    }
    // 理論上不會到這裡（因為一定能往上找到）
    -1
}

fn permute(
    cnt: &mut [i32; 10],
    len: i32,
    cur: i64,
    pos: i32,
    out: &mut Vec<i32>,
) {
    if pos == len {
        // 所有數字都在 1..7，因此 cur 一定在 i32 範圍內
        out.push(cur as i32);
        return;
    }
    for d in 1..=7 {
        let u = d as usize;
        if cnt[u] > 0 {
            cnt[u] -= 1;
            permute(cnt, len, cur * 10 + d as i64, pos + 1, out);
            cnt[u] += 1;
        }
    }
}