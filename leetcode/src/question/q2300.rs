use std::cmp::Ordering;

pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut result = Vec::with_capacity(spells.len());
    potions.sort();
    for (i, spell) in spells.iter().enumerate() {
        let mut left = 0;
        let mut right = potions.len();
        while left < right {
            let mid = (left+right) / 2;
            let prod = *spell as i64 * potions[mid] as i64;
            match prod.cmp(&success) {
                Ordering::Less => left = mid + 1,
                _ => right = mid,
            }
        }
        result.push((potions.len()-left) as i32);
    }
    result
}
