use std::collections::BTreeSet;

struct Parser<'a> {
    b: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn parse_expression(&mut self) -> BTreeSet<String> {
        let mut result = self.parse_term();
        while self.pos < self.b.len() && self.b[self.pos] == b',' {
            self.pos += 1;
            result.extend(self.parse_term());
        }
        result
    }
    fn parse_term(&mut self) -> BTreeSet<String> {
        let mut result = BTreeSet::new();
        result.insert(String::new());
        while self.pos < self.b.len() && self.b[self.pos] != b',' && self.b[self.pos] != b'}' {
            let factor = self.parse_factor();
            let mut next = BTreeSet::new();
            for prefix in &result {
                for suffix in &factor {
                    let mut word = String::with_capacity(prefix.len() + suffix.len());
                    word.push_str(prefix);
                    word.push_str(suffix);
                    next.insert(word);
                }
            }
            result = next;
        }
        result
    }
    fn parse_factor(&mut self) -> BTreeSet<String> {
        if self.b[self.pos] == b'{'{
            self.pos += 1;
            let result = self.parse_expression();
            self.pos += 1;
            result
        }else{
            let letter = (self.b[self.pos] as char).to_string();
            self.pos += 1;
            let mut result = BTreeSet::new();
            result.insert(letter);
            result
        }
    }
}

pub fn brace_expansion_ii(expression: String) -> Vec<String> {
    let mut parser = Parser {
        b: expression.as_bytes(),
        pos: 0,
    };
    parser.parse_expression().into_iter().collect()
}
