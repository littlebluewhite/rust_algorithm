struct Robot {
    width: i32,
    height: i32,
    path: Vec<(i32, i32)>,
    index: usize,
    moved: bool
}

impl Robot {

    fn new(width: i32, height: i32) -> Self {
        let mut path: Vec<(i32, i32)> = Vec::with_capacity((width+height) as usize *2 -4);
        for x in 0..width{
            path.push((x, 0));
        }
        for y in 1..height{
            path.push((width-1, y));
        }
        for x in (0..width-1).rev(){
            path.push((x, height-1));
        }
        for y in (1..height-1).rev(){
            path.push((0, y));
        }
        Self{
            width,
            height,
            path,
            index: 0,
            moved: false
        }
    }

    fn step(& mut self, num: i32) {
        let cycle = self.path.len();
        self.index = (self.index + (num as usize)%cycle) % cycle;
        self.moved = true;
    }

    fn get_pos(&self) -> Vec<i32> {
        let (x, y) = self.path[self.index];
        vec![x, y]
    }

    fn get_dir(&self) -> String {
        let (x, y) = self.path[self.index];
        let dir = if self.index == 0 {
            if self.moved{
                "South"
            }else{
                "East"
            }
        }else if y == 0{
            "East"
        }else if x == self.width - 1 {
            "North"
        }else if y == self.height - 1 {
            "West"
        }else{
            "South"
        };
        dir.to_string()
    }
}