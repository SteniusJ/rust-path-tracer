use crate::vec3;

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
     * Returns String containing complete ppm file.
     */
    pub fn to_string(&self) -> String {
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

        output
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
