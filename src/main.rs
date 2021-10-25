use clap::*;
use core::result::Result;
use raytrust::{render, write_pgm};
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
        Some(file) => Box::new(
            fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(file)
                .unwrap(),
        ),
        None => Box::new(io::stdout()),
    };

    // Sample image
    let image = render(800, 450);
    write_pgm(&mut *output, &image)
}
