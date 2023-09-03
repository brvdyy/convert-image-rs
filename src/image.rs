use clap::Parser;
use image::{imageops::FilterType, DynamicImage, GenericImageView};
use std::path::PathBuf;

use crate::args::Args;

pub struct Image {
    pub input: PathBuf,
    pub output: PathBuf,
    pub img: DynamicImage,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub filter: FilterType,
}

impl Image {
    pub fn new() -> Image {
        let args = Args::parse();
        Image {
            img: match image::open(&args.input) {
                Ok(image) => image,
                Err(error) => {
                    println!("There was a problem with the input file:\n{:?}", error);
                    std::process::exit(1);
                }
            },
            input: args.input,
            output: args.output,
            width: args.width,
            height: args.height,
            filter: FilterType::Triangle,
        }
    }

    pub fn save(self) {
        match self.img.save(&self.output) {
            Ok(image) => image,
            Err(error) => {
                println!("There was a problem converting the file:\n{:?}", error);
                std::process::exit(1);
            }
        };
    }

    pub fn scale(&mut self) {
        self.img = self
            .img
            .resize(self.width.unwrap(), self.height.unwrap(), self.filter);
    }
}
