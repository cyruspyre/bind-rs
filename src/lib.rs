use std::{
    fmt::{Display, Formatter, Result},
    ptr::null_mut,
};

#[derive(Debug)]
pub struct Bind {
    idx: usize,
    len: usize,
    cur: *mut Node,
    head: *mut Node,
    last: *mut Node,
}

const THRESHOLD: usize = if cfg!(feature = "unicode") { 15 } else { 3000 };

impl Bind {
    pub fn new(str: String) -> Self {
        let len = str.len();
        let tmp = Box::into_raw(Box::new(Node::new(str)));

        Self {
            len,
            idx: 0,
            cur: null_mut(),
            head: tmp,
            last: tmp,
        }
    }

    pub fn push(&mut self, str: &str) {
        self.len += str.len();
        unsafe { (*self.last).str += str }
    }

    pub fn push_front(&mut self, str: &str) {
        let head = unsafe { &mut *self.head };

        // I'm not sure if this will cause issues in the future.
        if head.str.len() <= str.len() * 15 {
            head.str.insert_str(0, str);
        } else {
            self.head = Box::into_raw(Box::new(Node {
                str: str.to_string(),
                next: self.head,
            }));
        }

        self.len += str.len();
    }

    pub fn insert(&mut self, mut idx: usize, str: &str) {
        if idx == self.len {
            return self.push(str);
        }

        if idx == 0 {
            return self.push_front(str);
        }

        assert!(idx < self.len);

        let (head, last, cur) = unsafe { (&mut *self.head, &mut *self.last, self.cur) };
        let node = if head.has(0, idx - self.idx, &mut idx) {
            self.idx = 0;
            head
        } else if last.has(self.len - last.str.len(), idx - self.idx, &mut idx) {
            self.idx = self.len - last.str.len();
            last
        } else {
            let mut cur = match unsafe { cur.as_mut() } {
                Some(v) if self.idx < idx => v,
                _ => {
                    self.idx = 0;
                    head
                }
            };

            while !cur.has(self.idx, idx - self.idx, &mut idx) {
                if cur.next.is_null() {
                    break;
                }

                let tmp = unsafe { &mut *cur.next };
                self.idx += cur.str.len();
                cur = tmp
            }

            cur
        };

        // I'm not sure if this will cause issues in the future.
        if node.str.len() - idx - self.idx <= THRESHOLD {
            self.cur = node;
            self.len += str.len();
            return node.str.insert_str(idx - self.idx, str);
        }

        if self.idx + node.str.len() != idx {
            let (a, b) = node.str.split_at(idx - self.idx);
            let tmp = Box::into_raw(Box::new(Node {
                str: b.to_string(),
                next: node.next,
            }));

            if self.idx + node.str.len() == self.len {
                self.last = tmp;
            }

            node.str = a.to_string();
            node.next = tmp;
        }

        node.str += str;
        self.cur = node;
        self.len += str.len();
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Display for Bind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut buf = String::with_capacity(self.len);
        let mut tmp = unsafe { &*self.head };

        loop {
            buf += &tmp.str;

            if tmp.next.is_null() {
                break;
            }

            tmp = unsafe { &*tmp.next };
        }

        write!(f, "{buf}")
    }
}

#[derive(Debug)]
struct Node {
    str: String,
    next: *mut Node,
}

impl Node {
    fn new(str: String) -> Self {
        Self {
            str,
            next: null_mut(),
        }
    }

    fn has(&self, start: usize, ldx: usize, idx: &mut usize) -> bool {
        #[cfg(feature = "unicode")]
        {
            for (i, c) in self.str.char_indices() {
                if i >= ldx {
                    break;
                }

                if c.is_ascii() {
                    continue;
                }

                *idx += c.len_utf8() - 1
            }
        }

        start <= *idx && *idx <= start + self.str.len()
    }
}
