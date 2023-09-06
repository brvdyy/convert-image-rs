use clap::Parser;
use image::{imageops::FilterType, DynamicImage};
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
    /// Initializes a new Image based off of supplied arguments
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

    /// Saves image in current state to output path in desired format
    fn save(&mut self) {
        match self.img.save(&self.output) {
            Ok(image) => image,
            Err(error) => {
                println!("There was a problem converting the file:\n{:?}", error);
                std::process::exit(1);
            }
        };
    }

    /// Scales image and updates the image in the struct
    /// Only scales if both a width AND height are supplied
    fn scale(&mut self) {
        match (self.width, self.height) {
            (Some(_), Some(_)) => {
                self.img = self
                    .img
                    .resize(self.width.unwrap(), self.height.unwrap(), self.filter);
            },
            (None, Some(_)) => {
                println!("Error: Please provide a width with the -w flag. Desired height and width are both required for resizing.");
                std::process::exit(1);
            }
            (Some(_), None) => {
                println!("Error: Please provide a height with the -h flag. Desired height and width are both required for resizing.");
                std::process::exit(1);
            }
            _ => (),
        }
    }

    /// Main image processsing function than handles all deaired modifications to image
    /// This saves the image to the output path following any modifications
    pub fn process(&mut self) {
        self.scale();

        self.save();
    }
}
