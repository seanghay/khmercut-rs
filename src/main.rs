use clap::Parser;
use crfsuite::Model;
use std::io::{stdin, stdout, BufRead, Result, Write};

mod khmercut;

#[derive(Parser, Debug)]
#[command(author = "Seanghay Yath", version, about = "Fast Khmer word segmentation tool", long_about = None)]
struct Args {
    #[arg(short, long, default_value = " ")]
    delimiter: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let model = Model::from_file("crf_ner_10000.crfsuite").unwrap();
    for line in stdin().lock().lines() {
        let input_str = line.unwrap();
        stdout()
            .write_all(
                khmercut::tokenize(&model, &input_str)
                    .join(&args.delimiter)
                    .as_bytes(),
            )
            .unwrap();
    }
    Ok(())
}
