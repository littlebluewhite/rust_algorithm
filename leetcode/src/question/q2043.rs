struct Bank {
    balance: Vec<i64>,
}

impl Bank {
    // 建立銀行（輸入為 0-indexed 的餘額陣列，但帳號是 1-indexed）
    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    // 內部小工具：把 1-indexed 的帳號轉成 Vec 索引；無效就回傳 None
    fn idx(&self, account: i32) -> Option<usize> {
        let n = self.balance.len() as i32;
        if account >= 1 && account <= n {
            Some((account as usize) - 1)
        } else {
            None
        }
    }

    // 轉帳：account1 -> account2 轉 money
    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let (Some(i1), Some(i2)) = (self.idx(account1), self.idx(account2)) else {
            return false;
        };
        if self.balance[i1] < money {
            return false;
        }
        // 先扣再加（此題沒有溢位風險條件，但仍按順序安全處理）
        self.balance[i1] -= money;
        self.balance[i2] += money;
        true
    }

    // 存款
    fn deposit(&mut self, account: i32, money: i64) -> bool {
        let Some(i) = self.idx(account) else { return false; };
        self.balance[i] += money;
        true
    }

    // 提款
    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        let Some(i) = self.idx(account) else { return false; };
        if self.balance[i] < money {
            return false;
        }
        self.balance[i] -= money;
        true
    }
}
