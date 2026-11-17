use std::collections::VecDeque;

pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as i64;
    let mut blocks: VecDeque<(i64, i64)> = VecDeque::new();
    let mut right = nums.len();
    let mut cost = 0i64;
    let mut ans = 0i64;
    for left in (0..nums.len()).rev() {
        let height = nums[left] as i64;
        let mut width = 1i64;
        while let Some(&(old_height, old_width)) = blocks.back(){
            if old_height > height{
                break;
            }
            blocks.pop_back();
            cost += (height-old_height)*old_width;
            width += old_width;
        }
        blocks.push_back((height, width));
        while cost > k{
            let front = blocks.front_mut().unwrap();
            cost -= front.0-nums[right-1] as i64;
            front.1 -= 1;
            if front.1 == 0{
                blocks.pop_front();
            }
            right -= 1;
        }
        ans += (right - left) as i64;
    }
    ans
}