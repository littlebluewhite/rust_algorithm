// Input: grid = [[3,4,5,1,3],[3,3,4,2,3],[20,30,200,40,10],[1,5,5,4,1],[4,3,2,2,5]]
// Output: [228,216,211]
// Explanation: The rhombus shapes for the three biggest distinct rhombus sums are depicted above.
// - Blue: 20 + 3 + 200 + 5 = 228
// - Red: 200 + 2 + 10 + 4 = 216
// - Green: 5 + 200 + 4 + 2 = 211

struct AnnotatedRhombus {
    ans: Vec<i32>,
}

impl AnnotatedRhombus {
    fn new() -> Self {
        AnnotatedRhombus { ans: vec![0, 0, 0] }
    }

    fn put(&mut self, x: i32) {
        if x > self.ans[0] {
            self.ans[2] = self.ans[1];
            self.ans[1] = self.ans[0];
            self.ans[0] = x;
        } else if x != self.ans[0] && x > self.ans[1] {
            self.ans[2] = self.ans[1];
            self.ans[1] = x;
        } else if x != self.ans[0] && x != self.ans[1] && x > self.ans[2] {
            self.ans[2] = x;
        }
    }

    fn get(&self) -> Vec<i32> {
        let mut result = Vec::with_capacity(3);
        for &i in &self.ans {
            if i > 0 {
                result.push(i);
            }
        }
        result
    }
}

pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
    let n = grid.len();
    let m = grid[0].len();
    let mut sum1 = vec![vec![0; m + 2]; n + 1];
    let mut sum2 = vec![vec![0; m + 2]; n + 1];
    for i in 0..n {
        for j in 0..m {
            sum1[i + 1][j + 1] = sum1[i][j] + grid[i][j];
            sum2[i + 1][j + 1] = sum2[i][j + 2] + grid[i][j];
        }
    }
    let mut ans = AnnotatedRhombus::new();
    for i in 0..n {
        for j in 0..m {
            ans.put(grid[i][j]);
            let mut k = i + 2;
            while k < n {
                let mid = (k - i) / 2;
                if mid > j || mid + j >= m {
                    break;
                }
                let up = (i, j);
                let down = (k, j);
                let left = (i + mid, j - mid);
                let right = (i + mid, j + mid);
                let sum = sum1[right.0 + 1][right.1 + 1] - sum1[up.0][up.1]
                    + sum2[left.0 + 1][left.1 + 1]
                    - sum2[up.0][up.1+2]
                    + sum2[down.0 + 1][down.1 + 1]
                    - sum2[right.0][right.1+2]
                    + sum1[down.0 + 1][down.1 + 1]
                    - sum1[left.0][left.1]
                    - grid[up.0][up.1]
                    - grid[down.0][down.1]
                    - grid[left.0][left.1]
                    - grid[right.0][right.1];
                ans.put(sum);
                k += 2;
            }
        }
    }
    ans.get()
}
//
// 0 0 0 0 0
// 0 1 2 3 0
// 0 4 5 6 0
// 0 7 8 9 0
// 0 0 0 0 0
