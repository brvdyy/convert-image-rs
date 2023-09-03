use std::path::PathBuf;
use clap::Parser;
use image::DynamicImage;

use crate::args::Args;

pub struct Image {
    pub input: PathBuf,
    pub output: PathBuf,
    pub img: DynamicImage,
}


impl Image {
    pub fn new() -> Image {
        let args = Args::parse();
        Image {
            input: args.input.clone(),
            output: args.output.clone(),
            img: match image::open(args.input.clone()) {
                Ok(image) => image,
                Err(error) => {
                    println!("There was a problem with the input file:\n{:?}", error);
                    std::process::exit(1);
                },
        }
    }
}
    
    pub fn save(self) {
        match self.img.save(self.output) {
            Ok(image) => image,
            Err(error) => {
                println!("There was a problem converting the file:\n{:?}", error);
                std::process::exit(1);
            },
        };
    }
    
    pub fn resize() {
            
    }
}

