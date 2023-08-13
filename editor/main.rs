use eframe::egui;
use image;
use image::{DynamicImage, GenericImageView, Pixel};
use egui::{Context, Ui, Image, TextureHandle, Response, Color32, Pos2, Vec2, Rect};

mod brush;

//fn load_image(path : &str) -> egui::ColorImage {
//    // TODO: open image in other thread, so GUI starts immediately
//    let img = image::open(path).unwrap();
//    let size = [img.width() as usize, img.height() as usize];
//    let imgbuf = img.into_rgba8();
//    let pixels = imgbuf.as_flat_samples();
//
//    egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())
//}

fn main() -> Result<(), eframe::Error> {
    const IMG_PATH : &str = "images/lime.jpg";
    let edit_target = image::open(IMG_PATH).expect("images/lime.jpg is always there");

    return eframe::run_native("Lime Editor", eframe::NativeOptions::default(),
            Box::new(|cc| {
                let lime_editor = LimeEditor::new(cc, edit_target);
                Box::<LimeEditor>::new(lime_editor)
            })
    );
}

const TEXTURE_OPTIONS : egui::TextureOptions = egui::TextureOptions::NEAREST;
struct LimeEditor {
    secret_canvas : DynamicImage,
    canvas : TextureHandle,
    canvas_rect : Rect,
    viewport_rect : Rect,
    brush : brush::Brush,
    patches : Vec<brush::Patch>,
}

impl LimeEditor {
    fn new(cc: &eframe::CreationContext, target : DynamicImage) -> Self {
        let canvas_content = LimeEditor::get_texture_content(&target);
        let canvas = cc.egui_ctx.load_texture("target",
                canvas_content, TEXTURE_OPTIONS);

        let wininfo = &cc.integration_info.window_info;
        let viewport_rect = Rect::from_points(
                &[Pos2::ZERO, wininfo.size.to_pos2()]);
        let canvas_rect = LimeEditor::get_initial_target_view_rect(
                canvas.aspect_ratio(), viewport_rect.size());
        let brush = brush::Brush::new(100, 20);

        LimeEditor {
            secret_canvas: target,
            canvas,
            canvas_rect,
            viewport_rect,
            brush,
            patches: vec!(),
        }
    }
}

impl eframe::App for LimeEditor {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // center target view when window size changed
        let wininfo = frame.info().window_info;
        if self.viewport_rect.size() != wininfo.size {
            self.canvas_rect = LimeEditor::get_initial_target_view_rect(
                    self.canvas.aspect_ratio(),
                    frame.info().window_info.size);
            self.viewport_rect = Rect::from_points(
                &[Pos2::ZERO, wininfo.size.to_pos2()]);
        }

        if !self.patches.is_empty() {
            LimeEditor::update_canvas(&mut self.canvas,
                    &self.secret_canvas, &self.patches);
            self.patches.clear();
        }

        let target_window = egui::Window::new("")
            .title_bar(false) // no traditional window with title bar
            .frame(egui::Frame::none()) // and frame
            .fixed_pos(self.canvas_rect.min)
            .fixed_size(self.canvas_rect.size());

        target_window.show(ctx, |ui| self.add_target_window_contents(ui));

        let viewport = egui::CentralPanel::default();
        let viewport_resp = viewport.show(ctx, |ui| self.add_viewport_contents(ui));
        self.handle_viewport_response(viewport_resp.response);
    }
}

impl LimeEditor {
    fn update_canvas(canvas : &mut TextureHandle,
            secret_canvas : &DynamicImage, patches : &Vec<brush::Patch>) {
        for patch in patches {
            let canvas_at_patch = secret_canvas.view(patch.x,
                    patch.y, patch.width, patch.height);
            // only dereferenced SubImage is a GenericImage which we need in the
            // following (see SubImage documentation)
            let canvas_at_patch = *canvas_at_patch;

            let new_content = LimeEditor::get_texture_content(&canvas_at_patch);
            canvas.set_partial([patch.x as usize, patch.y as usize],
                    new_content, TEXTURE_OPTIONS);
        }
    }

    fn get_texture_content<I : GenericImageView>(img: &I) -> egui::ColorImage
    where
        I : GenericImageView,
        I::Pixel : Pixel<Subpixel = u8>
    {
        let content_size = [img.width(), img.height()].map(|x| x as usize);
        let mut content = egui::ColorImage::new(content_size, Color32::BLACK);

        for (x, y, pixel) in img.pixels() {
            let image::Rgba(rgba,) = pixel.to_rgba();
            content.pixels[(y * img.width() + x) as usize] =
                Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);
        }

        return content;
    }

    fn get_initial_target_view_rect(content_aspect_ratio : f32,
            window_size : Vec2) -> Rect {
        let center = (0.5 * window_size).to_pos2();

        let mut size = window_size;
        if content_aspect_ratio > 1.0 {
            size.y = size.x / content_aspect_ratio;
        } else {
            size.x = size.y * content_aspect_ratio;
        }

        return Rect::from_center_size(center, size);
    }


    fn add_target_window_contents(&mut self, ui : &mut Ui) {
        let canvas = Image::new(&self.canvas,
                self.canvas_rect.size());
        let canvas_resp = ui.add(canvas.sense(egui::Sense::click_and_drag()));
        self.handle_target_view_response(canvas_resp);

        ui.input(|i| self.handle_input(i));
    }

    fn handle_target_view_response(&mut self, resp : Response) {
        if resp.dragged_by(egui::PointerButton::Middle) {
            self.translate_target_view(resp.drag_delta());
        }
    }

    fn handle_input(&mut self, state: &egui::InputState) {
        // TODO: should we use hardcoded zoom levels as in GIMP?
        //      also note that GIMP zoom levels are not linear, but pretty arbitrary
        //      looking values
        if state.modifiers.ctrl && state.zoom_delta() > 1.0 {
            let expand_amount = (state.zoom_delta() - 1.0)
                * self.canvas_rect.size();
            self.canvas_rect = self.canvas_rect.expand2(expand_amount);
        } else if state.modifiers.ctrl && state.zoom_delta() < 1.0 {
            let shrink_amount = (1.0 - state.zoom_delta())
                * self.canvas_rect.size();
            self.canvas_rect = self.canvas_rect.shrink2(shrink_amount);
        }

        if state.pointer.button_pressed(egui::PointerButton::Primary) {
            let pos = state.pointer.interact_pos().unwrap();
            if self.canvas_rect.contains(pos) {
                self.apply_brush(pos);
            }
        }
    }

    fn apply_brush(&mut self, pos_on_window: Pos2) {
        if !self.canvas_rect.contains(pos_on_window) {
            return
        }

        let pos_on_window = (pos_on_window - self.canvas_rect.min)
            / (self.canvas_rect.max - self.canvas_rect.min);

        let pos_on_canvas = pos_on_window * self.canvas.size_vec2();
        let pos_on_canvas = [pos_on_canvas.x as u32, pos_on_canvas.y as u32];

        let patch;
        match &mut self.secret_canvas {
            DynamicImage::ImageRgb8(x) => {
                patch = self.brush.apply(x, pos_on_canvas);
            },
            _ => panic!(),
        }

        self.patches.push(patch);
    }

    fn add_viewport_contents(&mut self, _ui : &mut Ui) {
        // empty
    }

    fn handle_viewport_response(&mut self, resp : Response) {
        let resp = resp.interact(egui::Sense::drag());
        if resp.dragged_by(egui::PointerButton::Middle) {
            self.translate_target_view(resp.drag_delta());
        }
    }

    fn translate_target_view(&mut self, delta : Vec2) {
        self.canvas_rect = self.canvas_rect.translate(delta);
    }
}
