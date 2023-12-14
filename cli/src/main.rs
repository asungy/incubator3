use entity_core::viewport::Viewport;

fn main() {
    let mut viewport = Viewport::new(64, 64);
    viewport.put_pixel(0., 0., (255, 0, 0));
    viewport.put_pixel(-0.25, 0.5, (0, 255, 0));
    viewport.save_image("./image.png");
}
