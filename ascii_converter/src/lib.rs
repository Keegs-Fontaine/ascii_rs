use wasm_bindgen::prelude::*;

use std::fmt::Write;

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

#[wasm_bindgen]
pub fn get_ascii_img(file_data: &[u8], scaling_factor: u32) -> String {
    let img_res = image::load_from_memory(file_data);

    // It's not necessarily the best practice to do this, but I explicitly want the program to panic and error out if the image doesn't exist,
    // or if it can't decode it

    return match img_res {
        Err(e) => format!("Err! Couldn't get image: {}", e),
        Ok(img) => {
            let mut ascii_str = String::new();

            let resized = img.resize(
                img.width() / scaling_factor,
                img.height() / scaling_factor,
                imageops::FilterType::Lanczos3,
            );

            resized.to_rgb8().enumerate_pixels().for_each(|(x, _, p)| {
                let ch = p.to_ch();

                write!(ascii_str, "{}", ch).expect("Err! Couldn't write char to file!");

                if x == resized.width() - 1 {
                    ascii_str
                        .write_char('\n')
                        .expect("Err! Couldn't write a newline char!");
                }
            });

            ascii_str
        }
    };
}
