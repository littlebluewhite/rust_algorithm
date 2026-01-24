struct Event {
    y: i64,
    x1: i64,
    x2: i64,
    delta: i32,
}

impl Event {
    fn new(y: i64, x1: i64, x2: i64, delta: i32) -> Self {
        Event { y, x1, x2, delta }
    }
}

struct SweepSegment {
    y_start: f64,
    y_end: f64,
    start_area: f64,
    width: f64,
}

impl SweepSegment {
    fn new(y_start: f64, y_end: f64, start_area: f64, width: f64) -> Self {
        SweepSegment {
            y_start,
            y_end,
            start_area,
            width,
        }
    }
}

struct SegmentTree {
    n: usize,
    tree: Vec<f64>,
    count: Vec<i32>,
    xs: Vec<i64>,
}

impl SegmentTree {
    fn new(xs: Vec<i64>) -> Self {
        let n = xs.len() - 1;
        let size = n * 4;
        SegmentTree {
            n,
            tree: vec![0.0; size],
            count: vec![0; size],
            xs,
        }
    }

    fn update(&mut self, idx: usize, l: usize, r: usize, ql: usize, qr: usize, delta: i32) {
        if qr < l || ql > r {
            return;
        } else if ql <= l && qr >= r {
            self.count[idx] += delta;
        } else {
            let mid = (l + r) / 2;
            self.update(idx * 2, l, mid, ql, qr, delta);
            self.update(idx * 2 + 1, mid, r, ql, qr, delta);
        }
        if self.count[idx] > 0 {
            self.tree[idx] = (self.xs[r] - self.xs[l]) as f64;
        } else if r - l == 1 {
            self.tree[idx] = 0.0;
        } else {
            self.tree[idx] = self.tree[idx * 2] + self.tree[idx * 2 + 1];
        }
    }

    fn query(&self) -> f64 {
        self.tree[1]
    }
}

pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
    let mut events: Vec<Event> = Vec::with_capacity(squares.len() * 2);
    let mut xs: Vec<i64> = Vec::with_capacity(squares.len() * 2);
    for sq in squares {
        let x = sq[0] as i64;
        let y = sq[1] as i64;
        let l = sq[2] as i64;
        events.push(Event::new(y, x, x + l, 1));
        events.push(Event::new(y + l, x, x + l, -1));
        xs.push(x);
        xs.push(x + l);
    }
    events.sort_by_key(|e| e.y);
    xs.sort();
    xs.dedup();

    let mut st = SegmentTree::new(xs.clone());
    let mut segments: Vec<SweepSegment> = Vec::new();
    let mut total_area = 0.0;
    let mut i = 0usize;
    let mut pre_y = events[0].y;
    while i < events.len() {
        let cur_y = events[i].y;
        if cur_y > pre_y {
            let dy = (cur_y - pre_y) as f64;
            let width = st.query();
            segments.push(SweepSegment::new(
                pre_y as f64,
                cur_y as f64,
                total_area,
                width,
            ));
            total_area += width * dy;
        }
        while i < events.len() && events[i].y == cur_y{
            let l = xs.binary_search(&events[i].x1).unwrap();
            let r = xs.binary_search(&events[i].x2).unwrap();
            st.update(1, 0, st.n, l, r, events[i].delta);
        }
        pre_y = cur_y;
    }

    let target = total_area / 2.0;
    for seg in segments{
        let seg_area = seg.width * (seg.y_end - seg.y_start);
        if target <= seg.start_area + seg_area{
            return if seg.width == 0.0{
                seg.y_start
            }else{
                seg.y_start + (target-seg.start_area) / seg.width
            };
        }
    }

    pre_y as f64
}
