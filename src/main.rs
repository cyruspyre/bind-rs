use bind::Bind;

fn main() {
    let mut bind = Bind::new("HelloWorld".into());

    bind.push("!");
    bind.push_front("> ");
    bind.push_at(7, " ");

    println!("{:?}\n{bind}", bind);
}
