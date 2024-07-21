use image::{io::Reader as ImageReader, GenericImageView};
use colored::Colorize;

fn main() {
    let ascii = vec!["  ", "..","::", "cc", "oo", "PP", "OO", "??", "@@", "##"];

    let mut image = ImageReader::open("input.png").expect("Err").decode().expect("Err");
    image = image.resize(
        image.width() / 32, 
        image.height() / 32, 
        image::imageops::FilterType::Nearest
    );

    for y in 0..image.height() {
        for x in 0..image.width() {
            let color = image.get_pixel(x, y).0;
            let greyscale = image.grayscale().get_pixel(x, y).0;
            let luminance = get_luminance(greyscale[0] as f32 as u8, greyscale[1] as f32 as u8, greyscale[2] as f32 as u8);
            let index = match (luminance * 10.0).floor() / 10.0 {
                1.0 => 0,
                0.9 => 1,
                0.8 => 2,
                0.7 => 3,
                0.6 => 4,
                0.5 => 5,
                0.4 => 6,
                0.3 => 7,
                0.2 => 8,
                0.1 => 9,
                0.0 => 0,
                _ => 0,
            };

            print!("{}", ascii[index].truecolor(color[0], color[1], color[2]));
            //print!("{}", ascii[index].truecolor(color[0] / 3, color[1] / 3, color[2] / 3).on_truecolor(color[0], color[1], color[2]));
        }
        print!("\n");
    }
}

fn get_luminance(r: u8, g: u8, b: u8) -> f32 {
    (0.2126 * r as f32 + 0.7152 as f32 * g as f32 + 0.0722 * b as f32) / 255.0
}
