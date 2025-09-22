use std::collections::HashMap;

pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut elements_frequency: HashMap<i32, i32> = HashMap::new();
    let mut max_frequency = 0;
    for &num in &nums {
        let freq = elements_frequency.entry(num).or_insert(0);
        *freq += 1;
        max_frequency = max_frequency.max(*freq);
    }
    let a = elements_frequency.values();
    result = elements_frequency
        .values()
        .filter(|&&f| f == max_frequency)
        .sum();
    result
}

pub fn max_frequency_elements_2(nums: Vec<i32>) -> i32 {
    let mut freq = [0; 101];
    let mut max_frequency = 0;

    for num in nums {
        freq[num as usize] += 1;
        max_frequency = max_frequency.max(freq[num as usize]);
    }
    freq.iter().filter(|&&f| f == max_frequency).sum()
}