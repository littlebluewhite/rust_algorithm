pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut ans = i64::MIN;
    let mut i = 0usize;
    while i < n{
        let mut j = i+1;
        while j < n && nums[j-1] < nums[j]{
            j += 1;
        }
        let p = j-1;
        if p == i{
            i+=1;
            continue;
        }
        let mut res = (nums[p]+ nums[p-1]) as i64;
        while j < n && nums[j-1] > nums[j]{
            res += nums[j] as i64;
            j +=1;
        }
        let q = j-1;
        if q == p || q == n-1 || (j < n && nums[j-1] == nums[j]){
            i = q;
            continue;
        }
        res += nums[q+1] as i64;

        let mut max_sum = 0i64;
        let mut sum = 0i64;
        let mut r = q+2;
        while r < n && nums[r-1] < nums[r]{
            sum += nums[r] as i64;
            max_sum = max_sum.max(sum);
            r += 1;
        }
        res+= max_sum;

        let mut l = p as isize -2;
        sum = 0;
        max_sum = 0i64;
        while l >= i as isize{
            sum += nums[l as usize] as i64;
            max_sum = max_sum.max(sum);
            l-=1;
        }
        res += max_sum;
        if res > ans{
            ans = res;
        }

        i = q;
    }
    ans
}