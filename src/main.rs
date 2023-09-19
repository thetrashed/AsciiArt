use image::GenericImageView;
use terminal_size::terminal_size;

use std::env;
use std::path::Path;

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    let mut img = image::open(Path::new(&file)).unwrap();
    let characters =
        String::from("`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$");

    let (term_width, term_height) = terminal_size().unwrap();
    let mut img_width = img.dimensions().0;
    let mut img_height = img.dimensions().1;

    if (term_width.0 as u32) < img_width {
        img = img.resize(term_width.0 as u32, img_height, image::imageops::Gaussian);
        img_width = img.dimensions().0;
        img_height = img.dimensions().1;
    }

    if (term_height.0 as u32) < img_height {
        img = img.resize(img_width, term_height.0 as u32, image::imageops::Gaussian);
        img_width = img.dimensions().0;
    }

    let brightness: Vec<_> = img
        .pixels()
        .map(|x| ((x.2[0] as u16 + x.2[1] as u16 + x.2[2] as u16) / 3) as u8)
        .collect();

    let converted_pixels: Vec<_> = brightness
        .iter()
        .map(|&x| characters.chars().nth((x as f64 / 20.0).round() as usize))
        .collect();

    for (count, pixel) in converted_pixels.iter().enumerate() {
        if count as u32 % img_width == 0 && count != 0 {
            print!("\n{}{}{}", pixel.unwrap(), pixel.unwrap(), pixel.unwrap());
        } else {
            print!("{}{}{}", pixel.unwrap(), pixel.unwrap(), pixel.unwrap());
        }
    }
    println!();
}
