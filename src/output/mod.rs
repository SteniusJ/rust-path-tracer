use crate::vec3;

pub struct RenderPPM {
    width: u16,
    height: u16,
    max_color: u8,
    pixels: Vec<PixelData>
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
    pub fn push(&mut self, color: vec3::Color) {
        self.pixels.push(PixelData::new(color.r, color.g, color.b));
    }
    pub fn to_string(&self) -> String {
        let mut output = String::with_capacity((self.width as usize * self.height as usize) * 15 + 15);

        output += &format!("P3\n{} {}\n{}\n", self.width, self.height, self.max_color);
        
        for pixel_data in &self.pixels {
            output += &pixel_data.to_string_ppm();
            println!("cap: {}", output.capacity());
            println!("len: {}", output.len());
        }

        output
    }
}

struct PixelData {
    red: u8,
    green: u8,
    blue: u8
}

impl PixelData {
    fn new(red: u8, green: u8, blue: u8) -> PixelData {
        PixelData { red, green, blue }
    }
    fn to_string_ppm(&self) -> String {
        format!("{} {} {}\n", self.red, self.green, self.blue)
    }
}
