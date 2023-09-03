use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub input: PathBuf,

    #[arg(short, long)]
    pub output: PathBuf
}

