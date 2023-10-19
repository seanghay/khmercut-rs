## khmercut.rs

A Blazingly Fast Khmer Word Segmentation Tool written in Rust.


Build

```shell
cargo build --release
# binary file: ./target/release/khmercut
```

---

## Usage in CLI

```shell
echo "ឃាត់ខ្លួនជនសង្ស័យ០៤នាក់ Hello, world ករណីលួចខ្សែភ្លើង នៅស្រុកព្រៃនប់។" | khmercut -d '|'

# => ឃាត់ខ្លួន|ជនសង្ស័យ|០៤|នាក់| |Hello,| |world| |ករណី|លួច|ខ្សែភ្លើង| |នៅ|ស្រុក|ព្រៃនប់|។|

# with file

khmercut < file.txt
```

## Rust

```rust
use std::fs;
use crfs:Model;

fn main() {
    let buf = fs::read("src/crf_ner_10000.crfsuite").unwrap();
    let model = Model::new(&buf).unwrap();
    let input_str = "ឃាត់ខ្លួនជនសង្ស័យ០៤នាក់ Hello, world ករណីលួចខ្សែភ្លើង នៅស្រុកព្រៃនប់។".to_string();
    for token in khmercut::tokenize(&model, &input_str) {
        print!("{}|", token);
    }
}
```

---

## References

- [VietHoang1512/khmer-nltk/](https://github.com/VietHoang1512/khmer-nltk/) Khmer Natural Language Processing Tookit, Phan Viet Hoang (2020)
- [seanghay/khmercut](https://github.com/seanghay/khmercut) A (fast) Khmer word segmentation toolkit.