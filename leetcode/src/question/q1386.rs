use std::collections::HashMap;

pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
    const LEFT:u16 = (1u16 << 2) | (1u16<<3) | (1u16<<4) | (1u16<<5);
    const MIDDLE:u16 = (1u16 << 4) | (1u16<<5) | (1u16<<6) | (1u16<<7);
    const RIGHT:u16 = (1u16 << 6) | (1u16<<7) | (1u16<<8) | (1u16<<9);
    let mut map: HashMap<i32, u16> = HashMap::with_capacity(reserved_seats.len());
    for seats in reserved_seats{
        let row = seats[0];
        let col = seats[1];
        if (2..10).contains(&col){
            *map.entry(row).or_insert(0) |= 1 << col;
        }
    }
    let mut ans = (n - map.len() as i32)*2;
    println!("{:?}", map);
    println!("{:?}", ans);
    for (&mask) in map.values(){
        let left = mask & LEFT;
        let middle = mask & MIDDLE;
        let right = mask & RIGHT;
        if left == 0 || middle == 0 || right == 0{
            ans += 1;
        }
    }
    ans
}