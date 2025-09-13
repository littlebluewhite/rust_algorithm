use std::collections::HashMap;
use std::collections::HashSet;

pub fn max_freq_sum(s: String) -> i32{
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let vowels_set: HashSet<char> = vowels.iter().cloned().collect();
    let mut freq: HashMap<char, i32> = HashMap::new();

    for c in s.chars(){
        *freq.entry(c).or_insert(0) += 1;
    }

    let mut max_vowel_freq = 0;
    let mut max_consonant_freq = 0;
    for (c, count) in freq{
        if vowels_set.contains(&c){
            max_vowel_freq = max_vowel_freq.max(count);
        }else{
            max_consonant_freq = max_consonant_freq.max(count);
        }
    }

    max_vowel_freq + max_consonant_freq
}