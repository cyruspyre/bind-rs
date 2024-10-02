pub mod line;

use std::{
    fmt::{Display, Formatter, Result},
    ops::{Bound, RangeBounds},
    ptr::{null, null_mut},
};

use line::Lines;

#[derive(Debug)]
pub struct Bind {
    len: usize,
    idx: Index,
    cur: *mut Node,
    head: *mut Node,
    last: *mut Node,
    is_last: bool,
}

impl Bind {
    pub fn new() -> Self {
        Self::build(String::new(), null_mut())
    }

    pub fn push(&mut self, str: &str) {
        self.len += str.len();
        unsafe { (*self.last).data += str }
    }

    pub fn push_front(&mut self, str: &str) {
        let head = unsafe { &mut *self.head };

        // I'm not sure if this will cause issues in the future.
        if head.data.len() <= 500 {
            head.data.insert_str(0, str);
        } else {
            self.head = Node::new(str.to_string(), self.head);
            self.cur = self.head;
        }

        self.len += str.len();
    }

    pub fn insert(&mut self, pos: usize, str: &str) {
        if pos == self.len {
            return self.push(str);
        }

        if pos == 0 {
            return self.push_front(str);
        }

        assert!(pos < self.len);

        let node = self.get_node(pos);

        // // I'm not sure if this will cause issues in the future.
        if node.data.len() <= 200 {
            self.cur = node;
            self.len += str.len();
            return node.data.insert_str(self.idx.local, str);
        }

        let data = node.data.split_off(self.idx.local);

        node.data += str;
        node.next = Node::new(data, node.next);

        if self.is_last {
            self.last = node.next;
        }

        self.len += str.len();
    }

    pub fn slice<R: RangeBounds<usize>>(&mut self, rng: R) -> String {
        let mut start = match rng.start_bound() {
            Bound::Included(n) => *n,
            _ => 0,
        };
        let end = match rng.end_bound() {
            Bound::Included(n) => n + 1,
            Bound::Excluded(n) => *n,
            _ => self.len,
        };

        assert!(start <= end);

        let mut len = end - start;

        if len == 0 {
            return String::new();
        }

        let mut node = self.get_node(start);
        let mut buf = String::new();

        start = self.idx.local;

        'main: while len != 0 {
            for c in node.data[start..].chars() {
                buf.push(c);
                len -= 1;

                if len == 0 {
                    break 'main;
                }
            }

            node = match unsafe { node.next.as_mut() } {
                Some(v) => v,
                _ => break,
            };
            start = 0;
        }

        buf
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn lines(&self) -> Lines<'_> {
        Lines {
            bind: self,
            cur: null(),
            idx: 0,
        }
    }

    fn get_node<'a>(&mut self, pos: usize) -> &'a mut Node {
        let cur = unsafe { &mut *self.cur };

        self.is_last = false;

        if cur.has(&mut self.idx, pos) {
            self.is_last = self.head == self.last;
            cur
        } else {
            let last = unsafe { &mut *self.last };
            let tmp = self.len - last.data.len();
            let mut idx = Index {
                cur: tmp,
                node: tmp,
                stamp: tmp,
                local: 0,
            };

            if last.has(&mut idx, pos) {
                self.idx = idx;
                self.cur = last;
                self.is_last = true;

                return last;
            }

            let mut node = if pos >= self.idx.cur {
                cur
            } else {
                self.idx = Index::default();
                unsafe { &mut *self.head }
            };

            while !node.has(&mut self.idx, pos) {
                if node.next.is_null() {
                    break;
                }

                self.idx.node = self.idx.cur;
                node = unsafe { &mut *node.next };
            }

            self.cur = node;

            node
        }
    }

    fn build(data: String, next: *mut Node) -> Self {
        let len = data.len();
        let tmp = Node::new(data, next);

        Self {
            len,
            idx: Index::default(),
            cur: tmp,
            head: tmp,
            last: tmp,
            is_last: false,
        }
    }
}

impl<'a> From<&'a str> for Bind {
    fn from(value: &'a str) -> Self {
        Self::build(value.into(), null_mut())
    }
}

impl Display for Bind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut buf = String::with_capacity(self.len);
        let mut tmp = unsafe { &*self.head };

        loop {
            buf += &tmp.data;

            if tmp.next.is_null() {
                break;
            }

            tmp = unsafe { &*tmp.next };
        }

        write!(f, "{buf}")
    }
}

#[derive(Debug)]
struct Index {
    cur: usize,
    node: usize,
    stamp: usize,
    local: usize,
}

impl Default for Index {
    fn default() -> Self {
        Self {
            cur: 0,
            node: 0,
            stamp: 0,
            local: 0,
        }
    }
}

#[derive(Debug)]
struct Node {
    data: String,
    next: *mut Node,
}

impl Node {
    fn new(data: String, next: *mut Node) -> *mut Node {
        Box::into_raw(Box::new(Node { data, next }))
    }

    fn has(&self, idx: &mut Index, pos: usize) -> bool {
        if idx.node != idx.stamp {
            idx.local = 0;
        }

        #[cfg(feature = "unicode")]
        if pos > idx.cur {
            for c in unsafe { self.data.get_unchecked(idx.local..).chars() } {
                if idx.cur == pos {
                    break;
                }

                idx.cur += 1;
                idx.local += c.len_utf8();
            }
        }

        #[cfg(not(feature = "unicode"))]
        {
            if idx.cur > pos {
                return false;
            }

            idx.cur += self.data.len() - idx.local;

            if idx.cur > pos {
                idx.cur -= idx.cur - pos;
            }

            idx.local = idx.cur - idx.node;
        }

        idx.stamp = idx.node;
        idx.cur == pos
    }
}
