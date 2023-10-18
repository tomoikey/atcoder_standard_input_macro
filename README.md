# 🎉 Atcoder Input Macro in Rust 🎉

### About:
This macro has been instrumental in reading standard input in Rust, offering support for various data types such as `numbers`, `arrays`, and `tuples`. 　
I trust it will enhance your competitive programming experience and make it even more **enjoyable**!🔥

---
### How to use
```rust
fn main() {
    include_input! {
        n: u8,
        strings1: [String; n],
        m: u8,
        strings2: [(String, u32); m],
        numbers: [u64; 3]
    };

    println!(
        "{}\n{:?}\n{}\n{:?}\n{:?}",
        n, strings1, m, strings2, numbers
    );
}
```

### Input
```shell
1
hoge1
4
hoge2 2
hoge3 3
hoge4 4
hoge5 5
1
2
3
```

### Output
```shell
["hoge1"]
4
[("hoge2", 2), ("hoge3", 3), ("hoge4", 4), ("hoge5", 5)]
[1, 2, 3]
```