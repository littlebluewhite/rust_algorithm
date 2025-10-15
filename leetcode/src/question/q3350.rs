pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n < 2 { return 0; }

    // left[i]: 最長嚴格遞增子陣列「以 i 結尾」的長度
    let mut left = vec![1; n];
    for i in 1..n {
        if nums[i - 1] < nums[i] {
            left[i] = left[i - 1] + 1;
        }
    }

    // right[i]: 最長嚴格遞增子陣列「從 i 開始」的長度
    let mut right = vec![1; n];
    for i in (0..n - 1).rev() {
        if nums[i] < nums[i + 1] {
            right[i] = right[i + 1] + 1;
        }
    }

    // 分隔點在 i 與 i+1 之間，兩段可取的最大 k 是 min(left[i], right[i+1])
    let mut ans = 0;
    for i in 0..n - 1 {
        let k = left[i].min(right[i + 1]);
        if k > ans { ans = k; }
    }
    ans as i32
}