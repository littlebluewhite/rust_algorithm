use std::cmp;

pub struct Spreadsheet {
    grid: Vec<[i32; 26]>, // rows × 26 (A..Z)
}

impl Spreadsheet {
    pub fn new(rows: i32) -> Self {
        let r = cmp::max(0, rows) as usize;
        // 每列初始化為 26 個 0
        Self { grid: vec!([0; 26]; r) }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        if let Some((r, c)) = Self::parse_cell(&cell) {
            if r < self.grid.len() && c < 26 {
                self.grid[r][c] = value;
            }
        }
    }

    fn reset_cell(&mut self, cell: String) {
        if let Some((r, c)) = Self::parse_cell(&cell) {
            if r < self.grid.len() && c < 26 {
                self.grid[r][c] = 0;
            }
        }
    }

    fn get_value(&self, formula: String) -> i32 {
        // 公式格式固定 "=X+Y"，X、Y 可能是 cell 或非負整數
        // 先去掉 '='，再以 '+' 分割為兩個項
        let s = formula.trim_start_matches('=');
        let mut parts = s.splitn(2, '+');
        let a = parts.next().unwrap_or("");
        let b = parts.next().unwrap_or("");

        self.eval_term(a) + self.eval_term(b)
    }

    // ------- helpers -------

    // 解析 A1、B23 這種 cell；回傳 (row_idx, col_idx)，皆為 0-based
    fn parse_cell(s: &str) -> Option<(usize, usize)> {
        let mut chars = s.chars();
        let col_ch = chars.next()?;
        if !col_ch.is_ascii_uppercase() {
            return None;
        }
        let col = (col_ch as u8 - b'A') as usize;

        let row_str: String = chars.collect();
        let row_1based: usize = row_str.parse().ok()?;
        if row_1based == 0 {
            return None; // 題目保證 1..=rows，不過這裡保守一點
        }
        Some((row_1based - 1, col))
    }

    // 取得一個「項」的值：若是數字就 parse；若是 cell 就查表（未設定也視為 0）
    fn eval_term(&self, term: &str) -> i32 {
        if term.as_bytes().first().map(|b| b.is_ascii_digit()).unwrap_or(false) {
            return term.parse::<i32>().unwrap_or(0);
        }
        if let Some((r, c)) = Self::parse_cell(term) {
            if r < self.grid.len() && c < 26 {
                return self.grid[r][c];
            }
        }
        0
    }
}