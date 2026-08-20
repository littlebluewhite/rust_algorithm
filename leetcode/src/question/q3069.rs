pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
    let mut arr1 = vec![nums[0]];
    let mut arr2 = vec![nums[1]];
    for &num in nums.iter().skip(2){
        if arr1[arr1.len()-1]>arr2[arr2.len()-1]{
            arr1.push(num);
        }else{
            arr2.push(num);
        }
    }
    arr1.extend(arr2);
    arr1
}