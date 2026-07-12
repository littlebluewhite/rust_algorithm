pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
    let mut values = arr.clone();
    values.sort_unstable();
    values.dedup();
    arr.into_iter()
        .map(|v| values.binary_search(&v).unwrap() as i32 + 1)
        .collect()
}
