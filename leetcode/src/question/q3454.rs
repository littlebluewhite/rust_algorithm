struct SegmentTree{
    xs: Vec<i64>,
    len: Vec<i64>,
    cnt: Vec<i32>
}

impl SegmentTree{
    fn new(xs: Vec<i64>) -> Self{
        let size = xs.len();
        Self{
            xs,
            len: vec![0; size*4],
            cnt: vec![0; size*4]
        }
    }
    fn update( & mut self, i:usize, l: usize, r: usize, ql: usize, qr: usize, d: i32){
        if qr <= l || ql >= r{
            return;
        }
        if ql <= l && qr >= r{
            self.cnt[i] += d;
        }else{
            let m = (l+r)/2;
            self.update(i*2, l, m, ql, qr, d);
            self.update(i*2+1, m, r, ql, qr, d);
        }
        self.len[i] = if self.cnt[i] > 0{
            self.xs[r] - self.xs[l]
        }else if r-l == 1{
            0
        }else{
            self.len[i*2] + self.len[i*2+1]
        };
    }
}
pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
    let n = squares.len();
    let mut xs = Vec::with_capacity(2*n);
    for i in 0..n{
        let x1 = squares[i][0] as i64;
        let x2 = x1 + squares[i][2] as i64;
        xs.push(x1);
        xs.push(x2);
    }
    xs.sort_unstable();
    xs.dedup();

    let mut events: Vec<(i64, usize, usize, i32)> = Vec::with_capacity(2*n);
    for i in 0..n{
        let x1 = squares[i][0] as i64;
        let y1 = squares[i][1] as i64;
        let l = squares[i][2] as i64;
        let x2 = x1 + l;
        let y2 = y1 + l;
        let a = xs.partition_point(|&x| x < x1);
        let b = xs.partition_point(|&x| x < x2);
        events.push((y1, a, b, 1));
        events.push((y2, a, b, -1));
    }
    events.sort_unstable();
    let xs_len = xs.len()-1;
    let mut seg_tree = SegmentTree::new(xs);
    let mut total = 0i64;
    let mut strips: Vec<(i64, i64, i64)> = Vec::with_capacity(2*n);
    for i in 0..events.len(){
        let (y, a, b, d) = events[i];
        seg_tree.update(1, 0, xs_len, a, b, d);
        if i+1<events.len(){
            let w = seg_tree.len[1];
            let y2 = events[i+1].0;
            total += w * (y2-y);
            strips.push((y, y2, w));
        }
    }
    let mut acc = 0;
    for i in 0..strips.len(){
        let (y1, y2, w) = strips[i];
        let area = (y2-y1)*w;
        if (acc + area)*2 >= total{
            return ((total as f64 /2.0 - acc as f64)/w as f64)+y1 as f64
        }
        acc += area;
    }
    unreachable!()
}
