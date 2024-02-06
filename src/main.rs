use clap::Parser;
use image::Rgba;
use imageproc::drawing::{draw_text_mut, text_size};
use rusttype::{Font, Scale};
use std::error::Error;
use std::path::PathBuf;

/// Generate a meme
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File (graphic) on which to write text.
    #[arg(short, long)]
    in_file: PathBuf,

    /// Text to write at the top of the image
    #[arg(short, long)]
    top_text: String,

    /// Text to write at the bottom of the image
    #[arg(short, long)]
    bottom_text: String,

    /// Filename to which the image should be written
    #[arg(short, long)]
    out_file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    println!("Opening {:?}", args.in_file);

    let mut image = image::open(args.in_file)?;

    println!("Adding top text {}", args.top_text);

    // let font = Vec::from(include_bytes!("/usr/share/fonts/truetype/msttcorefonts/impact.ttf") as &[u8]);
    let font = Vec::from(include_bytes!("../font/impact.ttf") as &[u8]);
    let font = Font::try_from_vec(font).unwrap();

    let intended_text_height = 72.0;
    let scale = Scale {
        x: intended_text_height,
        y: intended_text_height,
    };

    // top text
    let (text_width, _) = text_size(scale, &font, args.top_text.as_str());
    let text_center = text_width / 2;
    let image_center = image.width() as i32 / 2;

    draw_text_mut(
        &mut image,
        Rgba([255 as u8, 255 as u8, 255 as u8, 255 as u8]),
        image_center - text_center,
        0,
        scale,
        &font,
        args.top_text.as_str(),
    );

    println!("Adding bottom text {}", args.bottom_text);

    // bottom text
    let (text_width, text_height) = text_size(scale, &font, args.bottom_text.as_str());
    let text_center = text_width / 2;
    let image_center = image.width() as i32 / 2;
    // the / 20 adds a bottom spacing of 5% of the overall image height.
    let bottom_text_location = image.height() as i32 - (text_height + (image.height() as i32 / 20));

    draw_text_mut(
        &mut image,
        Rgba([255 as u8, 255 as u8, 255 as u8, 255 as u8]),
        image_center - text_center,
        bottom_text_location,
        scale,
        &font,
        args.bottom_text.as_str(),
    );

    println!("Writing output to file {:?}", args.out_file);

    image.save(args.out_file)?;

    Ok(())
}
