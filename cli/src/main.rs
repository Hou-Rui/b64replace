use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};

use anyhow::{Context, Result};
use clap::Parser;

use b64replace_core::{Base64Replacer};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    input: Option<String>, // input file (default: stdin)
    #[arg(short, long)]
    output: Option<String>, // output file (default: stdout)
    #[arg(short, long, default_value = "^{}$")]
    template: String, // regex capture template (must contain `{}`)
}

fn open_input_file(input: &Option<String>) -> Result<Box<dyn BufRead>> {
    if let Some(path) = input {
        let ctx = || format!("opening input file: {}", path);
        let file = File::open(&path).with_context(ctx)?;
        Ok(Box::new(BufReader::new(file)))
    } else {
        Ok(Box::new(BufReader::new(io::stdin())))
    }
}

fn open_output_file(output: &Option<String>) -> Result<Box<dyn Write>> {
    if let Some(path) = output {
        let ctx = || format!("creating output file: {}", path);
        let file = File::create(&path).with_context(ctx)?;
        Ok(Box::new(BufWriter::new(file)))
    } else {
        Ok(Box::new(BufWriter::new(io::stdout())))
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let input = open_input_file(&args.input)?;
    let mut output = open_output_file(&args.output)?;
    let mut replacer = Base64Replacer::new(args.template);
    replacer.replace_all(input, &mut output)
}
