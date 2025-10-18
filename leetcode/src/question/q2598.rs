use std::collections::HashMap;

pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
    // 1) 統計各餘數類別的數量（處理負數要用「正餘數」）
    let v = value as i64;
    let mut cnt: HashMap<i64, i32> = HashMap::new();
    for x in nums {
        let mut r = x as i64 % v;
        if r < 0 { r += v; } // 把負餘數轉為 [0, value-1]
        *cnt.entry(r).or_insert(0) += 1;
    }

    // 2) 由 0 逐步嘗試構成 MEX
    let mut mex: i64 = 0;
    loop {
        let r = mex % v;
        if let Some(c) = cnt.get_mut(&r) {
            if *c > 0 {
                *c -= 1;      // 消耗一個這個餘數類別
                mex += 1;     // 可以得到下一個 i
                continue;
            }
        }
        // 沒庫存了，停止
        break;
    }
    mex as i32
}