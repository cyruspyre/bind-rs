use std::ptr::null_mut;

#[derive(Debug)]
struct Bind {
    len: usize,
    head: *mut Node,
    last: *mut Node,
}

impl Bind {
    fn new(str: String) -> Box<Self> {
        let len = str.len();
        let tmp = Box::into_raw(Box::new(Node::new(str)));

        let tmp = Box::new(Self {
            len,
            head: tmp,
            last: tmp,
        });

        tmp
    }

    fn push(&mut self, str: &str) {
        self.len += str.len();
        unsafe { (*self.last).str += str }
    }

    fn push_front(&mut self, str: &str) {
        self.head = Box::into_raw(Box::new(Node {
            str: str.to_string(),
            next: self.head,
        }));
        self.len += str.len();
    }

    fn text(&self) -> String {
        let mut buf = String::with_capacity(self.len);
        let mut tmp = unsafe { &*self.head };

        loop {
            buf += &tmp.str;

            if tmp.next.is_null() {
                break;
            }

            tmp = unsafe { &*tmp.next };
        }

        buf
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
}

fn main() {
    let mut bind = Bind::new("HelloWorld".into());

    bind.push("!");
    bind.push_front("> ");

    println!("{:?}", bind);
    println!("{}", bind.text());
}
