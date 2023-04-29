//! A canvas is an explicitly defined region on which the renderer can act.
//!
//! The `write_pixel` and `read_pixel` methods allow for direct manipulation/reading of pixel data.
use image::RgbImage;

use super::Color;

#[derive(Clone, Debug)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixels: RgbImage,
}

impl Canvas {
    /// Constructs a new, blank canvas.
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = RgbImage::new(width, height);

        Self {
            width,
            height,
            pixels,
        }
    }

    /// Draws the given color to the pixel located at `(x, y)`.
    pub fn write_pixel(&mut self, x: u32, y: u32, c: Color) {
        let (r, g, b) = scale_colors(&c);
        let c = image::Rgb([r, g, b]);

        self.pixels.put_pixel(x, y, c);
    }

    pub(crate) fn read_pixel(&self, x: u32, y: u32) -> Color {
        let p = self.pixels.get_pixel(x, y);

        let r = p[0] as f64 / 255.0;
        let g = p[1] as f64 / 255.0;
        let b = p[2] as f64 / 255.0;
        Color(r, g, b)
    }

    /// Exports the formatted file as described by the `path` input.
    pub fn export(&self, path: &str) -> image::ImageResult<()> {
        let mut img = image::RgbImage::new(self.width as u32, self.height as u32);

        for (x, y, pix) in img.enumerate_pixels_mut() {
            let color = &self.read_pixel(x, y);
            let (r, g, b) = clamped_color_channels(color);
            *pix = image::Rgb([r, g, b]);
        }

        img.save(path)
    }
}

fn clamped_color_channels(color: &Color) -> (u8, u8, u8) {
    let r = color.r();
    let g = color.g();
    let b = color.b();

    let ir = (256.0 * r.clamp(0.0, 0.999)) as i32;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as i32;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as i32;

    (ir as u8, ig as u8, ib as u8)
}

/// Scales each color channel to be between 0 and 255.
fn scale_colors(color: &Color) -> (u8, u8, u8) {
    let r = color.r();
    let g = color.g();
    let b = color.b();

    let ir = (256.0 * r.clamp(0.0, 0.999)) as i32;
    let ig = (256.0 * g.clamp(0.0, 0.999)) as i32;
    let ib = (256.0 * b.clamp(0.0, 0.999)) as i32;

    (ir as u8, ig as u8, ib as u8)
}

#[cfg(test)]
mod canvas_tests {
    use super::*;

    #[test]
    fn can_write_pixels_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        canvas.write_pixel(2, 3, Color::red());

        assert_eq!(canvas.read_pixel(2, 3), Color::red());
        assert_eq!(canvas.read_pixel(3, 2), Color::black());
    }

    #[test]
    #[ignore = "I don't want to save a file every time I run this test."]
    fn can_save_canvas_files() {
        let mut canvas = Canvas::new(400, 200);
        for j in 0..200 {
            for i in 0..400 {
                let r = i as f64 / (canvas.width - 1) as f64;
                let g = j as f64 / (canvas.height - 1) as f64;
                let b = 0.25;
                canvas.write_pixel(i, j, Color(r, g, b));
            }
        }
        canvas.export("/tmp/test.png").unwrap();
    }
}
