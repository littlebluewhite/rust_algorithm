// 主程式庫模組，包含 LeetCode 相關模組與對外公開的函式。

use std::error::Error;
use std::fs;

// 匯入 leetcode 子模組，並公開三個子模組：w4, w10, w23。
pub mod leetcode{
    pub mod w4;
    pub mod w10;
    pub mod w23;
    pub mod w25;
}

pub mod book{
    pub mod c12;
    pub mod c13;
}