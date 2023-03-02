use crate::color::RGBA;

#[allow(clippy::too_many_arguments)]
pub fn fill_circle(
    surface: &mut [RGBA],
    width: u32,
    height: u32,
    center_x: i32,
    center_y: i32,
    r: u32,
    color: RGBA,
) {
    assert_eq!((width * height) as usize, surface.len());

    let mut p;
    let (mut xx, mut yy);
    let r = r as i32;
    let rr = r * r;
    let (width, height) = (width as i32, height as i32);
    let (mut rel_x, mut rel_y);

    // skips OOB
    let ys = ((center_y - r)..=(center_y + r)).filter(|y| y >= &0 && y < &height);
    let xs = ((center_x - r)..=(center_x + r)).filter(|x| x >= &0 && x < &width);

    for y in ys {
        rel_y = y - center_y;
        yy = rel_y * rel_y;
        for x in xs.clone() {
            rel_x = x - center_x;
            xx = rel_x * rel_x;
            if xx + yy <= rr {
                p = y * width + x;
                if p >= 0 {
                    surface[p as usize] = color;
                }
            }
        }
    }
}
