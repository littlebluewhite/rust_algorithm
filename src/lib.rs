// 主程式庫模組，包含 LeetCode 相關模組與對外公開的函式。

// 匯入 leetcode 子模組，並公開三個子模組：w4, w10, w23。
mod leetcode{
    pub mod w4;   // week 4 的題目與解答
    pub mod w10;  // week 10 的題目與解答
    pub mod w23;  // week 23 的題目與解答
}
// 匯入 w4 模組，方便在本檔案內直接使用
use leetcode::w4;

// 對外公開的函式，計算兩個已排序陣列的中位數。
// 實際邏輯委託給 w4 模組的 find_median_sorted_arrays 函式。
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    w4::find_median_sorted_arrays(nums1, nums2)
}

// 測試模組，包含單元測試。
mod tests {
    #[test]
    fn it_works() {
        // 可以在這裡新增測試案例
    }
}