## khmercut.rs

A Blazingly Fast Khmer Word Segmentation Tool written in Rust.

```rust
let input_str = "ឃាត់ខ្លួនជនសង្ស័យ០៤នាក់ Hello, world ករណីលួចខ្សែភ្លើង នៅស្រុកព្រៃនប់។".to_string();

for token in khmercut::tokenize(&input_str) {
    print!("{}|", token);
}

// => ឃាត់ខ្លួន|ជនសង្ស័យ|០៤|នាក់| |Hello,| |world| |ករណី|លួច|ខ្សែភ្លើង| |នៅ|ស្រុក|ព្រៃនប់|។|
```

Run

```shell
cargo run
```

