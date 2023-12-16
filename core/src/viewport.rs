pub struct Viewport {
    width: u32,
    height: u32,
    rgb_buffer: Vec<u8>,
}

impl Viewport {
    pub fn new(width: u32, height: u32) -> Self {
        Viewport {
            width,
            height,
            rgb_buffer: vec![0; (3 * width * height) as usize],
        }
    }

    /// Wrapper for `normal_to_screen`.
    fn ntos(&self, x: f32, y: f32) -> (u32, u32) {
        let max_x = (self.width - 1) as f32;
        let max_y = (self.height - 1) as f32;
        crate::math::normal_to_screen(0., 0., max_x, max_y, x, y)
    }

    /// Convert coordinates to buffer index.
    fn index(&self, x: u32, y: u32) -> usize {
        ( x * self.width * 3 + y * 3) as usize
    }

    /// Get pixel in RGB buffer.
    fn get_rgb(&self, x: u32, y: u32) -> (u8, u8, u8) {
        let r = self.rgb_buffer[self.index(x, y) + 0];
        let g = self.rgb_buffer[self.index(x, y) + 1];
        let b = self.rgb_buffer[self.index(x, y) + 2];

        (r, g, b)
    }

    /// Set pixel in RGB buffer.
    fn set_rgb(&mut self, x: u32, y: u32, rgb: (u8, u8, u8)) {
        let r_index = self.index(x, y) + 0;
        let g_index = self.index(x, y) + 1;
        let b_index = self.index(x, y) + 2;
 
        self.rgb_buffer[r_index] = rgb.0;
        self.rgb_buffer[g_index] = rgb.1;
        self.rgb_buffer[b_index] = rgb.2;
    }

    /// Put pixel in RGB buffer.
    ///
    /// Note: x and y coordinates are normalized coordinates.
    pub fn put_pixel(&mut self, x: f32, y: f32, rgb: (u8, u8, u8)) {
        let (xr, yr) = self.ntos(x, y);
        self.set_rgb(xr, yr, rgb);
    }

    pub fn draw_line1(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, rgb: (u8, u8, u8)) {
        let mut t: f32 = 0.;
        while t < 1. {
            let x = x0 + ((x1 - x0) * t);
            let y = y0 + ((y1 - y0) * t);

            self.put_pixel(x, y, rgb);

            t += 0.001;
        }
    }

    pub fn draw_line2(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, rgb: (u8, u8, u8)) {
        let mut x = x0;
        while x <= x1 {
            let t: f32 = (x - x0) / (x1 - x0);
            let y: f32 = y0 * (1. - t) + y1 * t;

            self.put_pixel(x, y, rgb);

            x += 0.0001;
        }
    }

    pub fn save_image<Q: AsRef<std::path::Path>>(&self, path: Q) -> anyhow::Result<()> {
        let mut image = image::RgbImage::new(
            self.width, self.height
        );

        for x in 0..self.width {
            for y in 0..self.height {
                let (r, g, b) = self.get_rgb(x, y);
                image.put_pixel(x, y, image::Rgb([r, g, b]));
            }
        }

        image.save_with_format(path, image::ImageFormat::Png)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {

}
