use image::{RgbaImage, Rgb};
use egui::{InputState, Rect, PointerButton, Pos2, vec2};

pub struct Manager {
    brush: Brush,
    brush_state: BrushState,
    stroke_splotch_spacing: u32,
}

impl Manager {
    pub fn new() -> Self {
        let brush = Brush::new(100, 50, image::Rgb([0, 255, 0]));
        Self { brush, brush_state: BrushState::NonTouching,
            stroke_splotch_spacing: 20 }
    }

    pub fn apply_input(&mut self, input: &InputState,
            canvas_rect : Rect, target: &mut RgbaImage) -> Option<Patch> {
        let min_splotch_distance = self.brush.radius as f32
            * self.stroke_splotch_spacing as f32 / 100.0;
        self.brush_state = Manager::update_brush_state(self.brush_state, input,
                min_splotch_distance, canvas_rect, target.dimensions());

        match self.brush_state {
            BrushState::Stroking { pos, splotch_ready: true, .. }
                => self.brush.apply(target, (pos.x as i64, pos.y as i64)),
            BrushState::Dabbing { pos }
                => self.brush.apply(target, (pos.x as i64, pos.y as i64)),
            BrushState::Stroking { splotch_ready: false, .. } => None,
            BrushState::NonTouching => None,
        }
    }

    fn update_brush_state(state: BrushState, input: &InputState,
            min_splotch_distance: f32, canvas_rect : Rect,
            target_dims : (u32, u32)) -> BrushState {
        let dabbing = input.pointer.button_clicked(PointerButton::Primary);
        let stroking = input.pointer.button_down(PointerButton::Primary)
            && input.pointer.is_decidedly_dragging();

        if !dabbing && !stroking {
            return BrushState::NonTouching;
        }

        let pos_on_viewport = input.pointer.interact_pos()
            .expect("we are either dabbing or stroking");
        let pos_on_target = Manager::pos_on_target(
                pos_on_viewport, canvas_rect, target_dims);

        if dabbing {
            return BrushState::Dabbing { pos: pos_on_target };
        }

        match state {
            BrushState::Stroking { pos: last_pos, stroked_distance, .. } => {
                let stroked_distance = stroked_distance
                    + (pos_on_target - last_pos).length();
                if stroked_distance > min_splotch_distance {
                    return BrushState::Stroking {
                        pos: pos_on_target,
                        stroked_distance: 0.0,
                        splotch_ready: true,
                    };
                } else {
                    return BrushState::Stroking {
                        pos: pos_on_target,
                        stroked_distance,
                        splotch_ready: false,
                    };
                }
            },
            _ => BrushState::Stroking {
                pos: pos_on_target,
                stroked_distance: 0.0,
                splotch_ready: true,
            }
        }
    }

    fn pos_on_target(pos_on_window: Pos2, canvas_rect: Rect,
            target_dims : (u32, u32)) -> Pos2 {
        let pos_on_window = (pos_on_window - canvas_rect.min)
            / (canvas_rect.max - canvas_rect.min);

        let target_dims = vec2(target_dims.0 as f32, target_dims.1 as f32);
        (pos_on_window * target_dims).to_pos2()
    }
}

#[derive(Clone, Copy, Debug)]
enum BrushState {
    Stroking {
        pos: Pos2,
        stroked_distance: f32,
        splotch_ready: bool,
    },
    Dabbing {
        pos: Pos2,
    },
    NonTouching,
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

    fn apply(&self, target : &mut RgbaImage, pos : (i64, i64)) -> Option<Patch> {
        let brush_blob = Brush::build_brush_blob(
                self.radius, self.hardness, self.color);

        let x_blob = pos.0 - brush_blob.width() as i64 / 2;
        let y_blob = pos.1 - brush_blob.height() as i64 / 2;
        image::imageops::overlay(target, &brush_blob, x_blob, y_blob);

        Brush::get_patch_from_blob_bounds((x_blob, y_blob),
            brush_blob.dimensions(), target.dimensions())
    }

    fn get_patch_from_blob_bounds((x, y) : (i64, i64), (width, height) : (u32, u32),
            (target_width, target_height) : (u32, u32)) -> Option<Patch> {
        if (x + width as i64) < 0 || (y + height as i64) < 0 {
            return None
        }
        let x = x.max(0) as u32;
        let y = y.max(0) as u32;

        if x > target_width || y > target_height {
            return None
        }
        let width = width.min(target_width - x);
        let height = height.min(target_height - y);

        return Some(Patch { x, y, width, height });
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

#[derive(Debug)]
pub struct Patch {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
