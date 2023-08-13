#[derive(Debug)]
pub struct Patch {
    pub x : u32,
    pub y : u32,
    pub width : u32,
    pub height : u32,
}

pub struct Brush {
    radius : u32, // in pixels
    pub spacing : u32, // ratio between consecutive brush strokes in percent of radius
}

impl Brush {
    pub fn new(radius : u32, spacing : u32) -> Self {
        Self { radius, spacing }
    }

    pub fn set_radius(&mut self, radius : u32) {
        self.radius = radius;
    }

    pub fn set_spacing(&mut self, spacing : u32) {
        self.spacing = spacing;
    }

    pub fn apply(&self, target : &mut image::RgbImage, pos : [u32; 2]) -> Patch {
        let brush_width = 2 * self.radius;
        let red_pixels : Vec<u8> = vec!(0;
                (4 * brush_width.pow(2)) as usize);
        let brush_pixels = image::ImageBuffer::from_vec(
                brush_width, brush_width, red_pixels).unwrap();
        image::imageops::overlay(target, &brush_pixels,
                pos[0] as i64, pos[1] as i64);

        Patch { x : pos[0], y : pos[1], width : brush_pixels.width(),
            height : brush_pixels.height() }
    }

    //fn compute_mask(radius : u32) -> image::GrayImage {
    //    let mut mask = image::GrayImage::new((2 * radius + 1) as u32,
    //            (2 * radius + 1) as u32);
    //    let center = (radius as u32, radius as u32);
    //    for (x, y, pixel) in mask.enumerate_pixels_mut() {
    //        let dist = ((x - center.0).pow(2) +
    //                (y - center.1).pow(2)) as f32;
    //        let dist = dist.sqrt();
    //        let alpha = 1.0 - dist / radius as f32;
    //        *pixel = image::Luma([alpha as u8]);
    //    }
    //    mask
    //}
}
