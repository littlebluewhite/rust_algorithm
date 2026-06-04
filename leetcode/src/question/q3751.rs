pub fn total_waviness(num1: i32, num2: i32) -> i32 {
    let mut ans = 0;
    for i in num1..num2+1 {
        ans += count_waviness(i);
    }
    ans
}

fn count_waviness(mut n: i32) -> i32 {
    let mut arr = Vec::new();
    while n > 0 {
        arr.push(n % 10);
        n /= 10;
    }
    if arr.len() < 3 {
        return 0;
    }
    let mut count = 0;
    for i in 1..arr.len()-1 {
        if (arr[i-1] > arr[i] && arr[i] < arr[i+1]) || (arr[i-1] < arr[i] && arr[i] > arr[i+1]){
            count += 1;
        }
    }
    count
}
