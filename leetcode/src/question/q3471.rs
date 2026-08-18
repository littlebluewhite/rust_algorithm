pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut seen = [0;51];
    for i in 0..n{
        seen[nums[i] as usize] += 1;
    }
    if k == n as i32{
        return nums.iter().max().unwrap().clone()
    }
    if k > 1 {
        let mut a = nums[0];
        let mut b = nums[n-1];
        if b > a{
            std::mem::swap(&mut a, &mut b);
        }
        if seen[a as usize] == 1{
            return a
        }else if seen[b as usize] == 1{
            return b
        }else{
            return -1
        }
    }else{
        for i in (0..51).rev(){
            if seen[i as usize] == 1{
                return i
            }
        }
    }
    -1
}