use crate::{Bind, Node};

pub struct Lines<'a> {
    pub(crate) idx: usize,
    pub(crate) cur: *const Node,
    pub(crate) bind: &'a Bind,
}

impl<'a> Iterator for Lines<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.is_null() {
            if self.idx != 0 {
                return None;
            }
            self.cur = self.bind.head;
        }

        let mut str = String::new();
        let vec = unsafe { str.as_mut_vec() };
        let node = unsafe { &*self.cur };
        let bytes = node.data.as_bytes();

        for i in self.idx..bytes.len() {
            let b = bytes[i];

            if b == b'\n' {
                self.idx = i + 1;
                return Some(str);
            }

            if b == b'\r' && bytes[i + 1] == b'\n' {
                continue;
            }

            vec.push(b);
        }

        self.cur = node.next;
        self.idx = match self.cur.is_null() {
            true => 1,
            _ => 0,
        };

        match str.is_empty() {
            true => None,
            _ => Some(str),
        }
    }
}
