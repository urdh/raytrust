use clap::*;
use core::result::Result;
use raytrust::{get_scene, render, write_pgm};
use std::{fs, io, str::FromStr};

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
    let width = usize::from_str(matches.value_of("width").unwrap()).unwrap();
    let height = usize::from_str(matches.value_of("height").unwrap()).unwrap();
    let samples = usize::from_str(matches.value_of("samples").unwrap()).unwrap();
    let depth = usize::from_str(matches.value_of("depth").unwrap()).unwrap();
    let render_pb = indicatif::ProgressBar::new_spinner().with_message("Rendering image");
    let render_cb = |row: usize| {
        render_pb.set_message(format!("Rendered line {}/{}", row, height));
        render_pb.tick()
    };
    let (camera, scene) = get_scene((width as f32) / (height as f32));
    let image = render(&scene, &camera, width, height, samples, depth, render_cb);
    render_pb.finish_with_message(format!("{} lines rendered!", height));

    // Write to file
    let save_pb = indicatif::ProgressBar::new_spinner().with_message("Saving image");
    let save_cb = |_: usize| save_pb.tick();
    write_pgm(&mut *output, &image, 2.2, save_cb)?;
    save_pb.finish_with_message("Image saved!");

    Ok(())
}
