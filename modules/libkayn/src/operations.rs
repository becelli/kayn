type Pixel = [u8; 3];
type ColorInt = u32;

fn get_color_integer_from_rgb(r: u8, g: u8, b: u8) -> ColorInt {
    (r as u32) << 16 | (g as u32) << 8 | (b as u32)
}

fn get_color_integer_from_gray(gray: u8) -> ColorInt {
    (gray as u32) << 16 | (gray as u32) << 8 | (gray as u32)
}

fn get_rgb_from_color_integer(color: u32) -> Pixel {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8]
}

pub fn grayscale(image: Vec<Pixel>) -> Vec<ColorInt> {
    let mut new_image: Vec<ColorInt> = Vec::new();
    image.iter().for_each(|pixel| {
        let gray_tone =
            (((pixel[0] as f32) + (pixel[1] as f32) + (pixel[2] as f32)) / 3.0).round() as u8;
        let color = get_color_integer_from_gray(gray_tone);
        new_image.push(color);
    });
    new_image
}

pub fn negative(image: Vec<Pixel>) -> Vec<ColorInt> {
    let mut new_image: Vec<ColorInt> = Vec::new();
    image.iter().for_each(|pixel| {
        let r = 255 - pixel[2];
        let g = 255 - pixel[1];
        let b = 255 - pixel[0];
        let color = get_color_integer_from_rgb(r, g, b);
        new_image.push(color);
    });
    new_image
}
pub fn _filter_nxn(image: Vec<Pixel>, filter: Vec<f32>, width: u32, height: u32) -> Vec<ColorInt> {
    let f_size = filter.len() as u32;
    let f_side = (f_size as f32).sqrt().round() as u32;
    let half = (f_side / 2) as u32;
    let mut new_image: Vec<ColorInt> = Vec::new();

    for x in half..(width - half) {
        for y in half..(height - half) {
            let mut new_pixel = [0f32; 3];
            for i in 0..f_size {
                let x_: u32 = x + (i % f_side) - half;
                let y_: u32 = y + (i / f_side) - half;
                let aux_pixel = image[(y_ * width + x_) as usize];
                aux_pixel.iter().rev().enumerate().for_each(|(ch, color)| {
                    new_pixel[ch] += *color as f32 * filter[i as usize];
                });
            }
            let color = get_color_integer_from_rgb(
                new_pixel[0] as u8,
                new_pixel[1] as u8,
                new_pixel[2] as u8,
            );
            new_image.push(color);
        }
    }
    new_image
}

pub fn median(image: Vec<Pixel>, distance: u32, width: u32, height: u32) -> Vec<ColorInt> {
    let f_size = (distance * 2 + 1).pow(2) as u32;
    let f_side = 2 * distance + 1;
    let half = (f_side / 2) as u32;
    let mut new_image: Vec<ColorInt> = Vec::new();

    for y in half..(height - half) {
        for x in half..(width - half) {
            let mut pixels: Vec<Pixel> = vec![[0u8; 3]; f_size as usize];
            for i in 0..f_size {
                let x_: u32 = x + (i % f_side) - half;
                let y_: u32 = y + (i / f_side) - half;
                let aux_pixel = image[(y_ * width + x_) as usize];
                pixels[i as usize] = aux_pixel;
            }

            pixels.sort_by(|a, b| {
                let a_ = get_color_integer_from_rgb(a[2], a[1], a[0]);
                let b_ = get_color_integer_from_rgb(b[2], b[1], b[0]);
                a_.partial_cmp(&b_).unwrap()
            });

            new_image.push(get_color_integer_from_rgb(
                pixels[f_size as usize / 2][2],
                pixels[f_size as usize / 2][1],
                pixels[f_size as usize / 2][0],
            ));
        }
    }
    new_image
}
