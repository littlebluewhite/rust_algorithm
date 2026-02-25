use std::cmp::Ordering;

pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
    arr.sort_by_key(|&x|(x.count_ones(), x));
    arr
}

pub fn sort_by_bits2(mut arr: Vec<i32>) -> Vec<i32> {
    arr.sort_by(|a, b|{
        let a_bit = a.count_ones();
        let b_bit = b.count_ones();
        match a_bit.cmp(&b_bit){
            Ordering::Equal => a.cmp(b),
            other => other,
        }
    });
    arr
}