pub fn asteroids_destroyed(mut mass: i32, mut asteroids: Vec<i32>) -> bool {
    asteroids.sort_unstable();
    let mut mass = mass  as i64;
    for v in asteroids{
        let v = v as i64;
        if mass < v{
            return false;
        }
        mass += v;
    }
    true
}