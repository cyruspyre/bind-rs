# bind

`bind_rs` an experimental data structure for text manipulation, designed with memory and performance in mind. To be honest, I think it’s just a glorified linked list with extra goodies :) **Wait, hear me out!**

## Features

- **Flexible String Manipulation**: Easily insert at an arbitrary position without fearing character boundary
- **Unicode Support**: Optional support for Unicode characters
- **Minimal Memory Overhead**: Very minimal memory usage due to certain methods
- **Performance**: Even with unicode support, it has little to no performance hit. For better performace, consider enabling LTO

## Usage

Here's a quick example of how to use `bind_rs`:

```rust
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

assert_eq!(bind.to_string(), "✨ H e l l o. 💀  W o r l d !");
assert_eq!(bind.len(), 32); // Length in bytes

let bind = Bind::from("Line 1️⃣\nLine 2️⃣\n\n");
let mut lines = bind.lines();

assert_eq!(lines.next().unwrap(), "Line 1️⃣");
assert_eq!(lines.next().unwrap(), "Line 2️⃣");
assert_eq!(lines.next().unwrap(), "");
assert_eq!(lines.next(), None);
```

## Behind the Scenes

Internally, `bind_rs` uses a singly linked list to manage text, with an additional pointer to the most recently modified node. This minimizes the time required to perform operations by reducing the need to traverse the entire list for each modification.

To further optimize performance, `bind_rs` has a threshold for creating new nodes. Instead of creating a new node for every mutation, it attempts to use `String::insert_at()` when possible, reducing the need for new nodes. This helps minimize memory overhead and fragmentation, improving both memory efficiency and performance. However, I myself am a bit unsure if it'll backfire in the future.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.