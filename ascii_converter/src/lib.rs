use std::{fs::File, io::BufWriter, io::Write};

use image::ImageReader;

use image::imageops;
use image::Rgb;

trait AsciiPixel {
    fn to_gray(&self) -> u8;
    fn to_ch(&self) -> char;
}

impl AsciiPixel for Rgb<u8> {
    fn to_gray(&self) -> u8 {
        let [r, g, b] = self.0;
        (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8
    }

    fn to_ch(&self) -> char {
        let gray = self.to_gray();
        match gray {
            0..=25 => '@',
            26..=50 => '#',
            51..=75 => '*',
            76..=100 => '+',
            101..=125 => '=',
            126..=150 => '-',
            151..=175 => ':',
            176..=200 => '.',
            _ => ' ',
        }
    }
}

pub fn get_ascii_img(img_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let img = ImageReader::open(img_path)?.decode()?;
    // It's not necessarily the best practice to do this, but I explicitly want the program to panic and error out if the image doesn't exist,
    // or if it can't decode it

    let file = File::create("output.txt").unwrap();
    let mut writer = BufWriter::new(file);

    let resized = img.resize(
        img.width() / 10,
        img.height() / 10,
        imageops::FilterType::Lanczos3,
    );

    resized.to_rgb8().enumerate_pixels().for_each(|(x, _, p)| {
        let ch = p.to_ch();

        write!(writer, "{}", ch).expect("Err! Couldn't write char to file!");

        if x == resized.width() - 1 {
            writer
                .write_all(b"\n")
                .expect("Err! Couldn't write a newline char!");
        }
    });

    Ok(())
}
