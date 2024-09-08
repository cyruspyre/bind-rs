use bind_rs::Bind;

#[test]
fn insert() {
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
    bind.insert(11, " |");
    bind.insert(11, ".");

    assert_eq!(bind.to_string(), "> H e l l o. | W o r l d !");
}

#[test]
fn lines() {
    let bind = Bind::new("Line 1\nLine 2\nLine 3\n\n".into());
    let mut lines = bind.lines();

    assert_eq!(lines.next().unwrap(), "Line 1");
    assert_eq!(lines.next().unwrap(), "Line 2");
    assert_eq!(lines.next().unwrap(), "Line 3");
    assert_eq!(lines.next().unwrap(), "");
    assert_eq!(lines.next(), None);
}
