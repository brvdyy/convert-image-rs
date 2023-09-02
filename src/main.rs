use std::path::PathBuf;
use image::GenericImageView;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,

    #[arg(short, long)]
    output: PathBuf
}

fn main() {
    let args = Args::parse();

    println!("Input File: {:?}", args.input);
    println!("Output File: {:?}", args.output);

    let img = match image::open(args.input) {
        Ok(image) => image,
        Err(error) => panic!("There was a problem with the input file:\n{:?}", error),
    };
    
    println!("ColorType: {:?}", img.color());
    println!("Dimensions: {:?}", img.dimensions());

    match img.save(args.output) {
        Ok(image) => image,
        Err(error) => panic!("There was a problem converting the file:\n{:?}", error),
    };
}
