use std::env::args;

use image::{DynamicImage, GenericImage, Rgba};

fn main() {
    let args: Vec<_> = args().collect();
    let size = args[1].parse().unwrap_or(256);
    let xsize = size * 3 / 4;
    let iterations: u64 = args[2].parse().unwrap_or(255);
    let mut image = DynamicImage::new_rgb8(xsize, size);
    let size_rec = (size as f64).recip();
    for i in 0..xsize {
        for j in 0..size {
            let cx = i as f64 * 4.0 * size_rec - 2.0;
            let cy = j as f64 * 4.0 * size_rec - 2.0;
            let mut zx = 0.0;
            let mut zy = 0.0;
            let mut k = 0;
            while k < iterations {
                let zxq = zx * zx;
                let zyq = zy * zy;
                (zx, zy) = (zxq - zyq + cx, 2.0 * zx * zy + cy);
                if zxq + zyq >= 4.0 {
                    break;
                }
                k += 1;
            }
            let (rg, b) = if k == iterations {
                (0, 0)
            } else {
                ((k * 16).min(255) as u8, 255)
            };
            image.put_pixel(i, j, Rgba([rg, rg, b, 255]));
        }
    }
    image.save("mandelbrot.png").unwrap();
}
