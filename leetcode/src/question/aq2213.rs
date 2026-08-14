#[derive(Copy, Clone)]
struct Node {
    len: i32,
    prefix_char: u8,
    prefix_len: i32,
    suffix_char: u8,
    suffix_len: i32,
    best: i32,
}

impl Node {
    fn leaf(ch: u8) -> Self {
        Self {
            len: 1,
            prefix_char: ch,
            prefix_len: 1,
            suffix_char: ch,
            suffix_len: 1,
            best: 1,
        }
    }

    fn empty() -> Self {
        Self {
            len: 0,
            prefix_char: 0,
            prefix_len: 0,
            suffix_char: 0,
            suffix_len: 0,
            best: 0,
        }
    }

    fn merge(left: Self, right: Self) -> Self {
        let same_boundary = left.suffix_char == right.prefix_char;
        let prefix_len = {
            if same_boundary && left.prefix_len == left.len {
                left.prefix_len + right.prefix_len
            } else {
                left.prefix_len
            }
        };
        let suffix_len = {
            if same_boundary && right.suffix_len == right.len {
                right.suffix_len + left.suffix_len
            } else {
                right.suffix_len
            }
        };
        let mut best = left.best.max(right.best);
        if same_boundary {
            best = best.max(left.suffix_len + right.prefix_len);
        }
        Self {
            len: left.len + right.len,
            prefix_char: left.prefix_char,
            prefix_len,
            suffix_char: right.suffix_char,
            suffix_len,
            best,
        }
    }
}

struct SegmentTree {
    tree: Vec<Node>,
    n: usize,
}

impl SegmentTree {
    fn new(b: &[u8]) -> Self {
        let n = b.len();
        let mut tree = Self {
            tree: vec![Node::empty(); n * 4],
            n: b.len(),
        };
        tree.build(1, 0, n - 1, b);
        tree
    }

    fn build(&mut self, node: usize, left: usize, right: usize, b: &[u8]) {
        if left == right {
            self.tree[node] = Node::leaf(b[left]);
            return;
        }
        let mid = (left + right) / 2;
        self.build(node * 2, left, mid, b);
        self.build(node * 2 + 1, mid + 1, right, b);
        self.pull(node);
    }

    fn update(&mut self, index: usize, ch: u8) {
        self.update_insert(1, 0, self.n - 1, index, ch);
    }

    fn update_insert(&mut self, node: usize, left: usize, right: usize, index: usize, ch: u8) {
        if left == right {
            self.tree[node] = Node::leaf(ch);
            return;
        }
        let mid = (left + right) / 2;
        if index <= mid {
            self.update_insert(node * 2, left, mid, index, ch);
        } else {
            self.update_insert(node * 2 + 1, mid + 1, right, index, ch);
        }
        self.pull(node);
    }

    fn pull(&mut self, node: usize) {
        self.tree[node] = Node::merge(self.tree[node * 2], self.tree[node * 2 + 1]);
    }

    fn best(&self) -> i32 {
        self.tree[1].best
    }
}

pub fn longest_repeating(s: String, query_characters: String, query_indices: Vec<i32>) -> Vec<i32> {
    let mut tree = SegmentTree::new(s.as_bytes());
    let query_bytes = query_characters.as_bytes();
    let mut ans = Vec::with_capacity(query_indices.len());
    for (i, &index) in query_indices.iter().enumerate() {
        tree.update(index as usize, query_bytes[i]);
        ans.push(tree.best());
    }
    ans
}
