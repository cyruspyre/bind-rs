use bind_rs::Bind;

#[test]
fn insert() {
    let mut bind = Bind::from("HelloWorld");

    bind.push("!");
    bind.push_front("✨ ");
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
    bind.insert(11, " 💀 ");
    bind.insert(11, ".");

    assert_eq!(bind.to_string(), "✨ H e l l o. 💀  W o r l d !", "is the `unicode` feature on?");
}

#[test]
fn lines() {
    let bind = Bind::from("Line 1️⃣\nLine 2️⃣\nLine 3️⃣\n\n");
    let mut lines = bind.lines();

    assert_eq!(lines.next().unwrap(), "Line 1️⃣");
    assert_eq!(lines.next().unwrap(), "Line 2️⃣");
    assert_eq!(lines.next().unwrap(), "Line 3️⃣");
    assert_eq!(lines.next().unwrap(), "");
    assert_eq!(lines.next(), None);
}

#[test]
fn slice() {
    let mut bind = Bind::from("💀 Hello ✨ World 💀");

    assert_eq!(bind.slice(2..7), "Hello");
    assert_eq!(bind.slice(2..=6), "Hello");

    assert_eq!(bind.slice(10..15), "World");
    assert_eq!(bind.slice(10..=14), "World");
}