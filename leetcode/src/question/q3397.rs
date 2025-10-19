pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
    // 轉成 i64 以避免加減溢位
    let k = k as i64;

    // 對每個 x 構造區間 [x - k, x + k]
    let mut intervals: Vec<(i64, i64)> = nums
        .into_iter()
        .map(|x| {
            let x = x as i64;
            (x - k, x + k)
        })
        .collect();

    // 依右端點排序（右端點小的先）
    intervals.sort_by_key(|&(_, r)| r);

    // 貪心指標：下一個可用的最小整數
    // 用很小的起始值即可；實作上先設為 i64::MIN / 4 之類避免極端情況
    let mut cur: i64 = i64::MIN / 4;
    let mut ans: i64 = 0;

    for (l, r) in intervals {
        if cur < l {
            cur = l; // 至少要放在區間左端
        }
        if cur <= r {
            // 可以在這個區間放置 cur
            ans += 1;
            cur += 1; // 下一個整數必須不同
        }
    }

    ans as i32
}