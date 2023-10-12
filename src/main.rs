use std::io::Result;
mod khmercut;

fn main() -> Result<()> {
    let input_str = "ឃាត់ខ្លួនជនសង្ស័យ០៤នាក់ Hello, world ករណីលួចខ្សែភ្លើង នៅស្រុកព្រៃនប់។".to_string();
    for token in khmercut::tokenize(&input_str) {
        print!("{}|", token);
    }

    print!("\n");
    return Ok(());
}
