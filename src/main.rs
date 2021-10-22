use clap::*;
use core::result::Result;
use raytrust::{write_pgm, Image};
use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let yml = load_yaml!("main.yml");
    let app = App::from_yaml(yml)
        .about(crate_description!())
        .author(crate_authors!())
        .name(crate_name!())
        .version(crate_version!());
    let matches = app.get_matches();

    // Argument: output file (or stdout if "-")
    let mut output: Box<dyn io::Write> = match matches.value_of("output") {
        Some(file) => Box::new(fs::File::open(file).unwrap()),
        None => Box::new(io::stdout()),
    };

    // Sample image
    let image = Image::new(256, 256);
    write_pgm(&mut *output, &image)
}
