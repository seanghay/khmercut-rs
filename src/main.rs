use clap::Parser;
use crfs::Model;
use khmercut;
use std::io::{stdin, stdout, BufRead, Result, Write};

static MODEL_FILE: &'static [u8] = std::include_bytes!("crf_ner_10000.crfsuite");

#[derive(Parser, Debug)]
#[command(author = "Seanghay Yath", version, about = "Fast Khmer word segmentation tool", long_about = None)]
struct Args {
    #[arg(short, long, default_value = " ")]
    delimiter: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let model = Model::new(MODEL_FILE).unwrap();
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
