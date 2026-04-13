pub fn minimum_distance(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut last1 = vec![-1i32;n+1];
    let mut last2 = vec![-1i32;n+1];

    let mut ans = i32::MAX;
    for (i, &x) in nums.iter().enumerate(){
        let x = x as usize;
        let i = i as i32;
        if last2[x] != -1{
            ans = ans.min(2*(i - last2[x]));
        }
        last2[x] = last1[x];
        last1[x] = i;
    }
    if ans == i32::MAX {
        return -1;
    }
    ans
}
