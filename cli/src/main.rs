use entity_core::viewport::Viewport;

fn main() {
    let mut viewport = Viewport::new(1000, 2000);
    viewport.draw_line(-0.7, -0.6, 0.2, 0.8, (0, 0, 255));
    viewport.draw_line(-1.0, -1.0, 1.0, 1.0, (255, 0, 255));
    viewport.save_image("./image.png");
}
