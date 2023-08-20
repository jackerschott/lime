use eframe::egui;
use image;
use image::{RgbaImage, GenericImageView, Rgba};
use egui::{Context, Ui, Image, TextureHandle, Response, Color32, Pos2, Vec2, Rect};

mod brush;

fn main() -> Result<(), eframe::Error> {
    const IMG_PATH : &str = "images/lime.jpg";
    let edit_target = image::open(IMG_PATH).expect("images/lime.jpg is always there");
    let edit_target = edit_target.to_rgba8();

    return eframe::run_native("Lime Editor", eframe::NativeOptions::default(),
            Box::new(|cc| {
                let lime_editor = LimeEditor::new(cc, edit_target);
                Box::<LimeEditor>::new(lime_editor)
            })
    );
}

const TEXTURE_OPTIONS : egui::TextureOptions = egui::TextureOptions::NEAREST;
struct LimeEditor {
    secret_canvas : RgbaImage,
    canvas : TextureHandle,
    canvas_rect : Rect,
    viewport_rect : Rect,

    brush_manager : brush::Manager,
    patches : Vec<brush::Patch>,

    zoom_manager : ZoomManager,
}

impl eframe::App for LimeEditor {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        // center target view when window size changed
        let wininfo = frame.info().window_info;
        if self.viewport_rect.size() != wininfo.size {
            self.canvas_rect = self.zoom_manager.reset_zoom(
                    frame.info().window_info.size, self.canvas.aspect_ratio());
            self.viewport_rect = Rect::from_points(
                &[Pos2::ZERO, wininfo.size.to_pos2()]);
        }

        if !self.patches.is_empty() {
            Self::update_canvas(&mut self.canvas,
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
    // In principle one could use DynamicImage here for target to support a lot of
    // ways to represent color channels (i.e. Rgb, Rgba, Grayscale) and color channel
    // types (i.e. u8, u16, f32), which is something that e.g. GIMP supports; however
    // no need to be general here since egui does only support Rgba and u8 anyway
    // with its ColorImage
    fn new(cc: &eframe::CreationContext, target : RgbaImage) -> Self {
        let canvas_content = img_to_egui(&target);
        let canvas = cc.egui_ctx.load_texture("target",
                canvas_content, TEXTURE_OPTIONS);

        let wininfo = &cc.integration_info.window_info;
        let viewport_rect = Rect::from_points(
                &[Pos2::ZERO, wininfo.size.to_pos2()]);

        let mut zoom_manager = ZoomManager::new();
        let canvas_rect = zoom_manager.reset_zoom(
                viewport_rect.size(), canvas.aspect_ratio());

        LimeEditor {
            secret_canvas: target,
            canvas,
            canvas_rect,
            viewport_rect,
            brush_manager: brush::Manager::new(),
            patches: Vec::new(),
            zoom_manager,
        }
    }

    fn update_canvas(canvas : &mut TextureHandle,
            secret_canvas : &RgbaImage, patches : &Vec<brush::Patch>) {
        for patch in patches {
            let canvas_at_patch = secret_canvas.view(patch.x as u32,
                    patch.y as u32, patch.width, patch.height);
            // only dereferenced SubImage is a GenericImage which we need in the
            // following (see SubImage documentation)
            let canvas_at_patch = *canvas_at_patch;

            let new_content = img_to_egui(&canvas_at_patch);
            canvas.set_partial([patch.x as usize, patch.y as usize],
                    new_content, TEXTURE_OPTIONS);
        }
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
            self.canvas_rect = ZoomManager::translate(
                    self.canvas_rect, resp.drag_delta());
        }
    }

    fn handle_input(&mut self, state: &egui::InputState) {
        if state.pointer.hover_pos().is_some() && state.modifiers.ctrl
                && state.zoom_delta() != 1.0 {
            let pointer_pos = state.pointer.hover_pos()
                .expect("is_some checked above");
            self.canvas_rect = self.zoom_manager.zoom(self.canvas_rect,
                    self.viewport_rect.size(), state.zoom_delta(), pointer_pos);
        }

        if let Some(patch) = self.brush_manager.apply_input(state,
                self.canvas_rect, &mut self.secret_canvas) {
            self.patches.push(patch);
        }
    }

    fn add_viewport_contents(&mut self, _ui : &mut Ui) {
        // empty
    }

    fn handle_viewport_response(&mut self, resp : Response) {
        let resp = resp.interact(egui::Sense::drag());
        if resp.dragged_by(egui::PointerButton::Middle) {
            self.canvas_rect = ZoomManager::translate(
                    self.canvas_rect, resp.drag_delta());
        }
    }
}

fn img_to_egui<I>(img: &I) -> egui::ColorImage
where
    I : GenericImageView<Pixel = Rgba<u8>> // also support (derefed) SubImages
{
    let content_size = [img.width(), img.height()].map(|x| x as usize);
    let mut content = egui::ColorImage::new(content_size, Color32::BLACK);
    // pattern matching is awesome
    for (x, y, image::Rgba(rgba)) in img.pixels() {
        content.pixels[(y * img.width() + x) as usize] =
            Color32::from_rgba_unmultiplied(rgba[0], rgba[1], rgba[2], rgba[3]);
    }

    return content;
}

struct ZoomManager {
    zoom_level : u8,
}

impl ZoomManager {
    const ZOOM_FACTORS : [f32; 33] = [256.0, 180.0, 128.0, 90.0, 64.0, 45.0, 32.0,
        23.0, 16.0, 11.0, 8.0, 5.5, 4.0, 3.0, 2.0, 1.5, 1.0, 0.667, 0.5, 0.333, 0.25,
        0.182, 0.125, 0.0909, 0.0625, 0.0435, 0.0312, 0.0222, 0.0156, 0.0111,
        0.00781, 0.00556, 0.00391];

    fn new() -> Self {
        Self { zoom_level: Self::ZOOM_FACTORS.len() as u8 / 2 + 1 }
    }

    fn reset_zoom(&mut self, bound_size : Vec2, aspect_ratio : f32) -> Rect {
        self.zoom_level = Self::ZOOM_FACTORS.len() as u8 / 2 + 1;

        let size = Self::get_unscaled_size(bound_size, aspect_ratio);
        Rect::from_center_size((0.5 * bound_size).to_pos2(), size)
    }

    fn zoom(&mut self, rect : Rect, bound_size : Vec2,
            zoom_delta : f32, pointer_pos : Pos2) -> Rect {
        self.zoom_level = if zoom_delta > 1.0 {
            self.zoom_level.saturating_sub(1)
        } else {
            self.zoom_level.saturating_add(1)
        };

        let unscaled_size = Self::get_unscaled_size(
                bound_size, rect.aspect_ratio());

        let zoom_fac = Self::ZOOM_FACTORS[self.zoom_level as usize];
        let rel_zoom_fac = zoom_fac * unscaled_size.x / rect.width();
        Rect::from_center_size(
            pointer_pos + (rect.center() - pointer_pos) * rel_zoom_fac,
            rect.size() * rel_zoom_fac)
    }

    fn get_unscaled_size(bound_size : Vec2, aspect_ratio : f32) -> Vec2 {
        let mut size = bound_size;
        if aspect_ratio > 1.0 {
            size.y = size.x / aspect_ratio;
        } else {
            size.x = size.y * aspect_ratio;
        }
        return size;
    }

    fn translate(rect : Rect, drag_delta : Vec2) -> Rect {
        rect.translate(drag_delta)
    }
}
