use crate::color::RGBA;

#[allow(clippy::too_many_arguments)]
pub fn fill_rectangle(
    surface: &mut [RGBA],
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    color: RGBA,
) {
    assert_eq!((width * height) as usize, surface.len());

    let mut p: usize;
    let (width, height) = (width as i32, height as i32);

    for y in y..(y + h) {
        if y < 0 || y > height {
            continue;
        }
        for x in x..(x + w) {
            if x < 0 || x > width {
                continue;
            }
            p = (y * width + x) as usize;
            surface[p] = color;
        }
    }
}
