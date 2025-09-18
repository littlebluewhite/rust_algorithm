use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct FoodEntry {
    rating: i32,
    name: String,
}

impl Ord for FoodEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        // 大的評分排前面；若評分相同，字典序小的排前面
        match self.rating.cmp(&other.rating) {
            Ordering::Equal => other.name.cmp(&self.name), // 反向，因為 BinaryHeap 是 max-heap
            ord => ord,
        }
    }
}

impl PartialOrd for FoodEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct FoodRatings {
    food_info: HashMap<String, (String, i32)>,       // food -> (cuisine, rating)
    cuisine_map: HashMap<String, BinaryHeap<FoodEntry>>, // cuisine -> heap
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_info = HashMap::new();
        let mut cuisine_map: HashMap<String, BinaryHeap<FoodEntry>> = HashMap::new();

        for i in 0..foods.len() {
            food_info.insert(foods[i].clone(), (cuisines[i].clone(), ratings[i]));
            cuisine_map
                .entry(cuisines[i].clone())
                .or_default()
                .push(FoodEntry {
                    rating: ratings[i],
                    name: foods[i].clone(),
                });
        }

        Self {
            food_info,
            cuisine_map,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        // 先把 cuisine 拿出來
        let cuisine = self.food_info[&food].0.clone();

        // 更新 food_info
        self.food_info.insert(food.clone(), (cuisine.clone(), new_rating));

        // 新的 entry 丟進 heap
        self.cuisine_map.get_mut(&cuisine).unwrap().push(FoodEntry {
            rating: new_rating,
            name: food,
        });
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        let heap = self.cuisine_map.get_mut(&cuisine).unwrap();
        loop {
            let top = heap.peek().unwrap();
            let (cur_cuisine, cur_rating) = &self.food_info[&top.name];
            if *cur_cuisine == cuisine && *cur_rating == top.rating {
                return top.name.clone();
            }
            heap.pop(); // 過期的資料丟掉
        }
    }
}