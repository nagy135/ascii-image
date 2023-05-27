use image::GenericImageView;

/// gets perceived luminance, weighting different rgb parts
/// 0 to 1
fn luminance_percentil(r: u8, g: u8, b: u8) -> f32 {
    (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) / 255.0
}

fn main() {
    let img = image::open("resources/image.jpg").unwrap();

    let dimensions = img.dimensions();

    let mut result = "".to_string();

    for y in 0..dimensions.1 {
        for x in 0..dimensions.0 {
            let pixel = img.get_pixel(x, y);
            let lum = luminance_percentil(pixel[0], pixel[1], pixel[2]);
            if lum < 0.1 {
                result.push('.');
            } else if lum < 0.2 {
                result.push(',');
            } else if lum < 0.3 {
                result.push('i');
            } else if lum < 0.4 {
                result.push('o');
            } else if lum < 0.5 {
                result.push('b');
            } else if lum < 0.6 {
                result.push('H');
            } else if lum < 0.7 {
                result.push('G');
            } else if lum < 0.8 {
                result.push('@');
            } else if lum < 0.9 {
                result.push('&');
            } else {
                result.push('#');
            }
        }
        result.push('\n');
    }
    println!("result {}", result);
}
