use std::{
    fmt::{Display, Formatter, Result},
    ptr::null_mut,
};

#[derive(Debug)]
pub struct Bind {
    len: usize,
    cur: *mut Node,
    head: *mut Node,
    last: *mut Node,
}

impl Bind {
    pub fn new(str: String) -> Self {
        let len = str.len();
        let tmp = Box::into_raw(Box::new(Node::new(str)));

        Self {
            len,
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

        // 15 does magic!
        if head.str.len() <= str.len() * 15 {
            head.str.insert_str(0, str);
        } else {
            self.head = Box::into_raw(Box::new(Node {
                idx: 0,
                str: str.to_string(),
                next: self.head,
            }));
            unsafe { (*self.last).idx += str.len() }
        }
        self.len += str.len();
    }

    pub fn push_at(&mut self, idx: usize, str: &str) {
        if idx == self.len {
            return self.push(str);
        }

        if idx == 0 {
            return self.push_front(str);
        }

        assert!(idx < self.len);

        let (head, last, cur) = unsafe { (&mut *self.head, &mut *self.last, self.cur) };
        let node = if head.has(idx) {
            head
        } else if last.has(idx) {
            last
        } else {
            let mut cur = match unsafe { cur.as_mut() } {
                Some(v) if v.idx < idx => v,
                _ => head,
            };

            while !cur.has(idx) {
                if cur.next.is_null() {
                    break;
                }

                let tmp = unsafe { &mut *cur.next };
                tmp.idx = cur.idx + cur.str.len();
                cur = tmp
            }

            cur
        };

        if node.idx + node.str.len() != idx {
            let (a, b) = node.str.split_at(idx - node.idx);
            let tmp = Box::into_raw(Box::new(Node {
                idx: node.idx + a.len() + str.len(),
                str: b.to_string(),
                next: node.next,
            }));

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
    idx: usize,
    str: String,
    next: *mut Node,
}

impl Node {
    fn new(str: String) -> Self {
        Self {
            str,
            idx: 0,
            next: null_mut(),
        }
    }

    fn has(&self, i: usize) -> bool {
        self.idx <= i && i <= self.idx + self.str.len()
    }
}
