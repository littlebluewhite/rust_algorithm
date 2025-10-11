pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
    let mut result = i32::MIN;
    for i in 0..k {
        let mut count = 0;
        let mut j = (energy.len() as i32) - 1 - i;
        while j >= 0 {
            count += energy[j as usize];
            if count > result {
                result = count;
            }
            j -= k;
        }
    }
    result
}
