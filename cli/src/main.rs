use image::{RgbImage, Rgb};

// fn line(x0: u32, y0: u32, x1: u32, y1: u32, rgb_image: &mut RgbImage, color: Rgb<u8>) -> () {
//     let mut t: f32 = 0.;
//     while t < 1. {
//         let x = x0 + ((x1 - x0) * t);
//         let y = y0 + ((y1 - y0) * t);
//
//         rgb_image.put_pixel(x, y, color);
//
//         t += 0.01;
//     }
// }

struct Viewport {
    width: usize,
    height: usize,
    image: RgbImage,
}

impl Viewport {
    fn new(width: usize, height: usize) -> Self {
        let image = RgbImage::new(
            width.try_into().unwrap(),
            height.try_into().unwrap()
        );

        Viewport {
            width,
            height,
            image,
        }
    }

    fn put_pixel(&mut self, x: u32, y: u32, color: Rgb<u8>) -> () {
        self.image.put_pixel(x, y, color);
    }

    fn save<Q>(&self, path: Q) -> ()
    where Q: AsRef<std::path::Path>
    {
        self.image.save_with_format(path, image::ImageFormat::Png);
    }
}

fn main() {
    let mut viewport = Viewport::new(32, 32);

    viewport.put_pixel(5, 15, Rgb([255, 0, 0]));

    viewport.save("./image.png");
}
