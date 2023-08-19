use image::{RgbaImage, Rgb};
use egui::Pos2;

#[derive(Debug)]
pub struct Patch {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub struct BrushManager {
    brush: Brush,
    spacing: u32,
    dragged_distance: f32,
    last_probe_position: Option<egui::Pos2>,
    positive_last_check: bool,
}

impl BrushManager {
    pub fn new(brush: Brush, spacing: u32) -> Self {
        Self { brush, spacing, dragged_distance: 0.0,
            last_probe_position: None, positive_last_check: false }
    }

    //fn check_brush_application(&self, pos: Pos2) -> bool {
    //    let Some(lastpos) = self.last_probe_position else { return true };
    //    let distance = self.dragged_distance + (pos - lastpos).length();

    //    let needed_distance = self.brush.radius as f32 * self.spacing as f32 / 100.0;
    //    return distance >= needed_distance;
    //}

    pub fn probe_brush_application(&mut self, pos: Pos2) -> bool {
        let Some(lastpos) = self.last_probe_position else {
            self.last_probe_position = Some(pos);
            self.dragged_distance = 0.0;

            self.positive_last_check = true;
            return self.positive_last_check;
        };

        self.dragged_distance += (pos - lastpos).length();
        let needed_distance = self.brush.radius as f32 * self.spacing as f32 / 100.0;

        self.positive_last_check = self.dragged_distance >= needed_distance;
        return self.positive_last_check;
    }


    pub fn apply_brush(&self, target: &mut RgbaImage, pos: Pos2) -> Patch {
        assert!(self.positive_last_check);

        self.brush.apply(target, (pos.x as u32, pos.y as u32))
    }

}

pub struct Brush {
    radius : u32, // in pixels
    hardness : u32, // in percent of radius
    color : Rgb<u8>,
}

impl Brush {
    pub fn new(radius : u32, hardness : u32, color : Rgb<u8>) -> Self {
        Self { radius, hardness, color }
    }

    //pub fn set_radius(&mut self, radius : u32) {
    //    self.radius = radius;
    //}

    //pub fn set_spacing(&mut self, spacing : u32) {
    //    self.spacing = spacing;
    //}

    fn apply(&self, target : &mut RgbaImage, pos : (u32, u32)) -> Patch {
        let brush_blob = Brush::build_brush_blob(
                self.radius, self.hardness, self.color);

        let x_blob = pos.0 - brush_blob.width() / 2;
        let y_blob = pos.1 - brush_blob.height() / 2;
        image::imageops::overlay(target, &brush_blob, x_blob as i64, y_blob as i64);

        Patch { x : x_blob, y : y_blob,
            width : brush_blob.width(),
            height : brush_blob.height() }
    }

    fn build_brush_blob(radius : u32, hardness : u32,
            image::Rgb(color) : Rgb<u8>) -> RgbaImage {
        let brush_width = (2 * radius) as f32;
        let mut brush_blob = image::RgbaImage::new(
                brush_width as u32, brush_width as u32);
        for (i, j, image::Rgba(rgba)) in brush_blob.enumerate_pixels_mut() {
            // convert pixel indices (i, j) to (x, y) in [-1, 1]
            let x = (2.0 * i as f32 - brush_width) / brush_width;
            let y = (2.0 * j as f32 - brush_width) / brush_width;

            let r = (x.powi(2) + y.powi(2)).sqrt();

            let r_lower = hardness as f32 / 200.0;
            let r_upper = 1.0 - hardness as f32 / 200.0;
            if r < r_lower {
                rgba[0..3].clone_from_slice(&color);
                rgba[3] = 255;
                continue
            } else if r > r_upper {
                rgba[0..3].clone_from_slice(&color);
                rgba[3] = 0;
                continue
            }

            let opacity = get_rational_quadratic(
                    (r_lower, r_upper), 0.0, (1.0, 0.0), 0.0);

            rgba[0..3].clone_from_slice(&color);
            rgba[3] = (opacity(r) * 255.0) as u8;
        }

        return brush_blob;
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

// compute rational quadratic like this
//
//    ^     
//    |     slope0
// y1 | - - _____    
//    |          \____
//    |     |         \_
//    |                 \
//    |     |            \
//    |                   \_
//    |     |               \____     slope1
// y0 | - - - - - - - - - - - -  \_____
//    |     |                         |
//    |
//    |     |                         |
//    +------------------------------------->
//          x0                        x1
//
// see https://arxiv.org/abs/1906.04032, Eq. (4)
fn get_rational_quadratic((x1, x2) : (f32, f32), slope1 : f32,
        (y1, y2) : (f32, f32), slope2 : f32) -> impl Fn(f32) -> f32
{
    move |x| {
        // average slope
        let s = (y2 - y1) / (x2 - x1);

        // dimensionless and rescaled x
        let xi = (x - x1) / (x2 - x1);

        y1 + (y2 - y1) * (s * xi * xi + slope1 * xi * (1.0 - xi))
            / (s + (slope1 + slope2 - 2.0 * s) * xi * (1.0 - xi))
    }
}
