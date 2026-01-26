use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut nums: Vec<i64> = nums.into_iter().map(|x| x as i64).collect();
    let mut pre: Vec<i32> = (0..n).map(|i| i as i32 - 1).collect();
    let mut next: Vec<i32> = (0..n).map(|i| i as i32 + 1).collect();
    next[n - 1] = -1;
    let mut removed: Vec<bool> = vec![false; n];
    let mut d_count = 0i32;
    let mut heap: BinaryHeap<Reverse<(i64, i32)>> = BinaryHeap::new();
    for i in 0..n - 1 {
        if nums[i] > nums[i + 1] {
            d_count += 1;
        }
        let sum = nums[i] + nums[i + 1];
        heap.push(Reverse((sum, i as i32)));
    }
    if d_count == 0 {
        return 0;
    }
    let mut ans = 0i32;
    while d_count > 0 {
        let Reverse((sum, i)) = match heap.pop() {
            Some(x) => x,
            None => break,
        };
        let now_index = i as usize;
        let next_index = next[now_index];
        if next_index == -1
            || removed[next_index as usize]
            || removed[now_index]
            || nums[now_index] + nums[next_index as usize] != sum
        {
            continue;
        }
        let pre_index = pre[now_index];
        let next_next_index = next[next_index as usize];
        if pre_index != -1 && nums[pre_index as usize] > nums[now_index] {
            d_count -= 1;
        }
        if next_index != -1 && nums[now_index] > nums[next_index as usize] {
            d_count -= 1;
        }
        if next_next_index != -1 && nums[next_index as usize] > nums[next_next_index as usize] {
            d_count -= 1;
        }
        ans += 1;
        nums[now_index] = sum;
        next[now_index] = next_next_index;
        if next_next_index != -1 {
            pre[next_next_index as usize] = now_index as i32;
        }
        removed[next_index as usize] = true;
        if pre_index != -1 && nums[pre_index as usize] > sum {
            d_count += 1;
        }
        if next_next_index != -1 && sum > nums[next_next_index as usize] {
            d_count += 1;
        }
        if d_count == 0 {
            break;
        }
        if pre_index != -1 {
            heap.push(Reverse((nums[pre_index as usize] + sum, pre_index as i32)));
        }
        if next_next_index != -1 {
            heap.push(Reverse((
                nums[next_next_index as usize] + sum,
                now_index as i32,
            )));
        }
    }
    ans
}
