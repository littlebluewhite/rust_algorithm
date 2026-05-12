pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
    let mut new_task: Vec<(i32, i32)> = tasks
        .into_iter()
        .map(|t| (t[0], t[1]))
        .collect();
    new_task.sort_by(|&a, &b| (b.1 - b.0).cmp(&(a.1 - a.0)));

    let mut required = 0;
    let mut spent = 0;
    for (actual, minimin) in new_task {
        required = required.max(spent+minimin);
        spent += actual;
    }
    required
}