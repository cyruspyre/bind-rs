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
        self.head = Box::into_raw(Box::new(Node {
            idx: 0,
            str: str.to_string(),
            next: self.head,
        }));
        self.len += str.len();
        unsafe { (*self.last).idx += str.len() }
    }

    pub fn push_at(&mut self, i: usize, str: &str) {
        let (head, last, cur) = unsafe { (&mut *self.head, &mut *self.last, self.cur) };
        let node = if head.has(i) {
            head
        } else if last.has(i) {
            last
        } else {
            let mut cur = match cur.is_null() {
                true => head,
                _ => unsafe { &mut *cur },
            };

            while !cur.has(i) {
                if cur.next.is_null() {
                    break;
                }

                let tmp = unsafe { &mut *cur.next };
                tmp.idx = cur.idx + cur.str.len();
                cur = tmp
            }

            cur
        };

        if node.idx + node.str.len() != i {
            let (a, b) = node.str.split_at(i - node.idx);
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
