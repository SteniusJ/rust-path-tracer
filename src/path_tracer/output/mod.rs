#![allow(dead_code)]

use crate::path_tracer::vec3;
use std::io::prelude::*;
use std::io;
use std::fmt::Display;
use std::fmt;

pub struct RenderPPM {
    width: u16,
    height: u16,
    max_color: u8,
    pixels: Vec<PixelData8>
}

impl RenderPPM {
    pub fn new(width: u16, height: u16, max_color: u8) -> RenderPPM {
        RenderPPM {
            width,
            height,
            max_color,
            pixels: Vec::with_capacity(width as usize * height as usize)
        }
    }
    /*
     * Push new pixel data to RenderPPM
     */
    pub fn push(&mut self, color: vec3::Color) {
        self.pixels.push(PixelData8::new(color.r, color.g, color.b));
    }
    /*
     * Push gpu generated vector
     */
    pub fn push_gpu_vec(&mut self, pixels: Vec<(f64, f64, f64)>, samples: u8) {
        let n_pixels = pixels.len() / samples as usize;
        for i in 0..n_pixels {
            let mut color = vec3::Vec3::empty();

            // For each sample displace the position by n_pixels * sample to get the other samples
            for sample in 0..samples as usize {
                let pixel_data = pixels[i + n_pixels * sample];
                color += vec3::Vec3::new(pixel_data.0, pixel_data.1, pixel_data.2);
            }

            // Color normalization
            color /= samples as f64;
            color = vec3::Vec3::new(
                f64::sqrt(color.x),
                f64::sqrt(color.y),
                f64::sqrt(color.z)
                );
            let color = color.to_color();
            self.pixels.push(PixelData8::new(color.r, color.g, color.b));
        }
    }
    /*
     * Median filtering for denoising of image.
     */
    pub fn median_filter(&mut self, window_width: u8, progress_interval: i64) {
        let mut progress = 0.0;
        let edge = window_width as usize / 2;
        // reshape 1 dimensional pixel data to 2 dimensional representation for easier filtering
        let pixel_data: Vec<&[PixelData8]> = self.pixels.chunks(self.width as usize).collect();
        let mut new_pixels: Vec<PixelData8> = Vec::with_capacity(self.pixels.len() - (2 * self.height as usize + 2 * self.width as usize));

        for y in edge..self.height as usize - edge {
            for x in edge..self.width as usize - edge {
                let mut window: Vec<&PixelData8> = Vec::with_capacity(window_width as usize * window_width as usize);

                for fy in 0..window_width as usize {
                    for fx in 0..window_width as usize {
                        window.push(&pixel_data[y + fy - edge][x + fx - edge]);
                    }
                }

                // Get median of every color by sorting and taking the middle value
                window.sort_by_key(|pxd_a| pxd_a.red);
                let red = window[window.len() / 2].red;
                window.sort_by_key(|pxd_a| pxd_a.green);
                let green = window[window.len() / 2].green;
                window.sort_by_key(|pxd_a| pxd_a.blue);
                let blue = window[window.len() / 2].blue;

                new_pixels.push(PixelData8::new(red, green, blue));
            }

            progress += 100.0 / (self.height - edge as u16 * 2) as f64;
            if progress as i64 % progress_interval == 0 {
                print!("\rdenoise progress: {progress:.2}%");
                io::stdout().flush().unwrap();
            }
        }
        self.pixels = new_pixels;
        // Edge has to be removed from width and height since it isn't included in the new_pixels
        // values
        self.width -= edge as u16 * 2;
        self.height -= edge as u16 * 2;
    }
}

/*
 * Returns String containing complete ppm file.
 */
impl Display for RenderPPM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*
         * Creates output String with capacity for maximum possible length of String.
         * This size can only be reached if every pixel is a color where r,g and b are three
         * digits.
         * 
         * Function:
         * (px_width * px_height) * 12 + 6 + n digits in px_width + n digits in px_height + n digits in max_color
         *
         * 12 -> max number of characters in line defining pixel: "xxx xxx xxx\n" <- 12 characters (\n is one character)
         * 6 -> other characters in ppm header "P3\n \n\n" <- 6 characters
         */
        let w_digits = (self.width.checked_ilog10().unwrap_or(0) + 1) as usize;
        let h_digits = (self.width.checked_ilog10().unwrap_or(0) + 1) as usize;
        let mx_col_digits = (self.max_color.checked_ilog10().unwrap_or(0) + 1) as usize;
        let mut output = String::with_capacity((self.width as usize * self.height as usize) * 12 + 6 + w_digits + h_digits + mx_col_digits);

        output += &format!("P3\n{} {}\n{}\n", self.width, self.height, self.max_color);
        
        for pixel_data in &self.pixels {
            output += &pixel_data.to_string_ppm();
        }

        write!(f, "{output}")
    }
}

/*
 * Struct for storing 8 bit rgb color data. 
 */
struct PixelData8 {
    red: u8,
    green: u8,
    blue: u8
}

impl PixelData8 {
    fn new(red: u8, green: u8, blue: u8) -> PixelData8 {
        PixelData8 { red, green, blue }
    }
    /*
     * Creates string for ppm file format.
     */
    fn to_string_ppm(&self) -> String {
        format!("{} {} {}\n", self.red, self.green, self.blue)
    }
}
