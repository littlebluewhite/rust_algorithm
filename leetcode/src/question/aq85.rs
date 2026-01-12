pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
    if matrix.is_empty() || matrix[0].is_empty() {
        return 0;
    }
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut heights = vec![0i32; cols];
    let mut ans = 0i32;

    for r in 0..rows {
        for c in 0..cols {
            if matrix[r][c] == '1' {
                heights[c] += 1;
            } else {
                heights[c] = 0;
            }
        }
        ans = ans.max(largest_histogram(&heights));
    }
    ans
}

fn largest_histogram(heights: &Vec<i32>) -> i32 {
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area = 0i32;
    let mut extended = heights.clone();
    extended.push(0);

    for i in 0..extended.len() {
        let cur = extended[i];
        while let Some(&top) = stack.last() {
            if extended[top] <= cur {
                break;
            }
            let h = extended[top];
            stack.pop();
            let width = match stack.last() {
                Some(&left) => i - left - 1,
                None => i,
            };
            let area = h * width as i32;
            if area > max_area {
                max_area = area;
            }
        }
        stack.push(i);
    }
    max_area
}