use clap::Parser;
use core::result::Result;
use raytrust::{get_scene, render, write_pgm};
use std::{fs, io};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Output file (PGM format)
    #[arg(short, long)]
    output: Option<String>,

    /// Image width
    #[arg(long, default_value_t = 800)]
    width: usize,

    /// Image height
    #[arg(long, default_value_t = 450)]
    height: usize,

    /// Samples per pixel
    #[arg(long, default_value_t = 10)]
    samples: usize,

    /// Recursion depth
    #[arg(long, default_value_t = 50)]
    depth: usize,

    /// Rendered scene
    #[arg(long, default_value_t = String::from("small"))]
    scene: String,
}

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();

    // Argument: output file (or stdout if "-")
    let mut output: Box<dyn io::Write> = match cli.output {
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
    let width = cli.width;
    let height = cli.height;
    let samples = cli.samples;
    let depth = cli.depth;
    let render_pb = indicatif::ProgressBar::new_spinner().with_message("Rendering image");
    let render_cb = |row: usize| {
        render_pb.set_message(format!("Rendered line {}/{}", row, height));
        render_pb.tick()
    };
    let (camera, scene) = get_scene((width as f32) / (height as f32), cli.scene.as_str());
    let image = render(&scene, &camera, width, height, samples, depth, render_cb);
    render_pb.finish_with_message(format!("{} lines rendered!", height));

    // Write to file
    let save_pb = indicatif::ProgressBar::new_spinner().with_message("Saving image");
    let save_cb = |_: usize| save_pb.tick();
    write_pgm(&mut *output, &image, 2.2, save_cb)?;
    save_pb.finish_with_message("Image saved!");

    Ok(())
}
