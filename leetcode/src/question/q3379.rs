pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans: Vec<i32> = Vec::with_capacity(n);
    for i in 0..n{
        let n = n as i32;
        let mut index = i as i32 + nums[i];
        println!("index1: {}", index);
        index %= n;
        println!("index2: {}", index);
        index += n;
        println!("index3: {}", index);
        index %= n;
        println!("index4: {}", index);
        ans.push(nums[index as usize])
    }
    ans
}