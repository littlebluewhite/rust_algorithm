pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
    let mut drunk = num_bottles;
    let mut empty = num_bottles;

    while empty >= num_exchange {
        // 交換一次：花掉 k 個空瓶，喝完後空瓶 +1，門檻 +1
        empty -= num_exchange;
        drunk += 1;
        empty += 1;
        num_exchange += 1;
    }
    drunk
}