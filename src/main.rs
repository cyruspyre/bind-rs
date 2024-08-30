use bind::Bind;

fn main() {
    let mut bind = Bind::new("HelloWorld".into());

    bind.push("!");
    bind.push_front("> ");
    bind.insert(3, " ");
    bind.insert(5, " ");
    bind.insert(7, " ");
    bind.insert(9, " ");
    bind.insert(11, " ");
    bind.insert(13, " ");
    bind.insert(15, " ");
    bind.insert(17, " ");
    bind.insert(19, " ");
    bind.insert(21, " ");

    println!("{:?}\n{bind}", bind);
}
