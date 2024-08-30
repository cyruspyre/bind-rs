use bind::Bind;

fn main() {
    let mut bind = Bind::new("HelloWorld".into());

    bind.push("!");
    bind.push_front("> ");
    bind.insert_at(3, " ");
    bind.insert_at(5, " ");
    bind.insert_at(7, " ");
    bind.insert_at(9, " ");
    bind.insert_at(11, " ");
    bind.insert_at(13, " ");
    bind.insert_at(15, " ");
    bind.insert_at(17, " ");
    bind.insert_at(19, " ");
    bind.insert_at(21, " ");

    println!("{:?}\n{bind}", bind);
}
