/// Converts normalized coordinate to screen coordinate.
pub fn normal_to_screen(
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
    x: f32,
    y: f32,
) -> (u32, u32) {
    assert!(max_x > min_x);
    assert!(max_y > min_y);

    let xr = (max_x - min_x) / 2. * (x + 1.);
    let yr = (max_y - min_y) / 2. * (-y + 1.);

    (xr as u32, yr as u32)
}


