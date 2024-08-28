use clap::Parser;
use crfs::Model;
use std::fs::File;
use khmercut;
use std::io::{stdin, stdout, BufRead, BufReader, Result, Write};

static MODEL_FILE: &'static [u8] = std::include_bytes!("crf_ner_10000.crfsuite");

#[derive(Parser, Debug)]
#[command(author = "Seanghay Yath", version, about = "Fast Khmer word segmentation tool", long_about = None)]
struct Args {
    #[arg(short, long, default_value = " ")]
    delimiter: String,

    #[arg(short, long)]
    text: Option<String>,

    #[arg(short, long)]
    file: Option<String>,
}

fn process_and_write(model: &Model, input: &String, delimiter: &str) -> Result<()> {
    let tokens = khmercut::tokenize(model, input);
    let output = tokens.join(delimiter);
    stdout().write_all(output.as_bytes())?;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let model = Model::new(MODEL_FILE).unwrap();

    if let Some(input_text) = args.text {
        process_and_write(&model, &input_text, &args.delimiter)?;
    }
    else if let Some(input_file) = args.file {
        let file = File::open(input_file)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let input_str = line?;
            process_and_write(&model, &input_str, &args.delimiter)?;
        }
    }
    else {

        for line in stdin().lock().lines() {
            let input_str = line.unwrap();
            process_and_write(&model, &input_str, &args.delimiter)?;
        }
    }

    Ok(())
}
