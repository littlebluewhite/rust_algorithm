use std::collections::{BTreeSet, HashMap};

pub struct MovieRentingSystem {
    // 取得某 (shop, movie) 的 price
    price: HashMap<(i32, i32), i32>,
    // 每部電影目前可租的分店集合（依 price, shop 排序）
    available: HashMap<i32, BTreeSet<(i32, i32)>>,
    // 全域已出租集合（依 price, shop, movie 排序）
    rented: BTreeSet<(i32, i32, i32)>,
}

impl MovieRentingSystem {
    pub fn new(_n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut price = HashMap::new();
        let mut available: HashMap<i32, BTreeSet<(i32, i32)>> = HashMap::new();

        for e in entries {
            let (shop, movie, p) = (e[0], e[1], e[2]);
            price.insert((shop, movie), p);
            available.entry(movie).or_default().insert((p, shop));
        }

        Self {
            price,
            available,
            rented: BTreeSet::new(),
        }
    }

    // 回傳最多 5 個分店 id
    pub fn search(&self, movie: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        if let Some(set) = self.available.get(&movie) {
            for &(_p, shop) in set.iter().take(5) {
                ans.push(shop);
            }
        }
        ans
    }

    pub fn rent(&mut self, shop: i32, movie: i32) {
        let p = *self.price.get(&(shop, movie)).unwrap();
        if let Some(set) = self.available.get_mut(&movie) {
            set.remove(&(p, shop)); // 從可租移除
        }
        self.rented.insert((p, shop, movie)); // 加入已出租
    }

    pub fn drop(&mut self, shop: i32, movie: i32) {
        let p = *self.price.get(&(shop, movie)).unwrap();
        self.rented.remove(&(p, shop, movie)); // 從已出租移除
        self.available.entry(movie).or_default().insert((p, shop)); // 放回可租
    }

    // 回傳最多 5 筆 [shop, movie]
    pub fn report(&self) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        for &(_p, shop, movie) in self.rented.iter().take(5) {
            ans.push(vec![shop, movie]);
        }
        ans
    }
}