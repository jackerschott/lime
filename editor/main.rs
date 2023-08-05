use eframe::egui;

//mod viewport;

fn load_image(path : &str) -> egui::ColorImage {
    // TODO: open image in other thread, so GUI starts immediately
    let img = image::open(path).unwrap();
    let size = [img.width() as usize, img.height() as usize];
    let imgbuf = img.into_rgba8();
    let pixels = imgbuf.as_flat_samples();

    egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())
}

fn main() -> Result<(), eframe::Error> {
    const IMG_PATH : &str = "images/lime.jpg";
    let edit_target = load_image(IMG_PATH);

    return eframe::run_native("Lime Editor", eframe::NativeOptions::default(),
            Box::new(|cc| {
                let lime_editor = LimeEditor::new(cc, edit_target);
                Box::<LimeEditor>::new(lime_editor)
            })
    );
}

struct LimeEditor {
    _target : egui::ColorImage,
    target_view_content : egui::TextureHandle,
    target_view_rect : Option<egui::Rect>,
}

impl LimeEditor {
    fn new(cc: &eframe::CreationContext, target : egui::ColorImage) -> Self {
        let target_view_content = cc.egui_ctx.load_texture("target",
                target.clone(), Default::default()).clone();
        LimeEditor { _target : target, target_view_content, target_view_rect: None }
    }
}

impl eframe::App for LimeEditor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // the window has its initial default size on the first frame (which can be
        // eg. manually entered) and it has only responded to external resize
        // requests (by eg. the window manager) on the second frame;
        // since we need frame.info().window_info.size below, skip the first frame
        // TODO: this is some obvious bullshit, find a better solution here
        if ctx.frame_nr() == 0 {
            return
        }

        let target_view_rect = self.target_view_rect.get_or_insert_with(|| {
            LimeEditor::get_initial_target_view_rect(
                    self.target_view_content.aspect_ratio(),
                    frame.info().window_info.size)
        });

        let target_window = egui::Window::new("")
            .title_bar(false) // no traditional window with title bar
            .frame(egui::Frame::none()) // and frame
            .fixed_pos(target_view_rect.min)
            .fixed_size(target_view_rect.size());

        target_window.show(ctx, |ui| self.add_target_window_contents(ui));

        let viewport = egui::CentralPanel::default();
        let viewport_resp = viewport.show(ctx, |ui| self.add_viewport_contents(ui));
        self.handle_viewport_response(viewport_resp.response);
    }
}

impl LimeEditor {
    fn get_initial_target_view_rect(content_aspect_ratio : f32,
            window_size : egui::Vec2) -> egui::Rect {
        let center = (0.5 * window_size).to_pos2();

        let mut size = window_size;
        if content_aspect_ratio > 1.0 {
            size.y = size.x / content_aspect_ratio;
        } else {
            size.x = size.y * content_aspect_ratio;
        }

        return egui::Rect::from_center_size(center, size);
    }

    fn add_target_window_contents(&mut self, ui : &mut egui::Ui) {
        let target_view_rect = self.target_view_rect.expect(
                "is set early every update call");
        let target_view = egui::Image::new(&self.target_view_content,
                target_view_rect.size());
        let target_view_resp = ui.add(target_view.sense(
                    egui::Sense::click_and_drag()));
        self.handle_target_view_response(target_view_resp);

        ui.input(|i| self.handle_input(i));
    }

    fn handle_target_view_response(&mut self, resp : egui::Response) {
        if resp.dragged_by(egui::PointerButton::Middle) {
            self.translate_target_view(resp.drag_delta());
        }
    }

    fn handle_input(&mut self, state: &egui::InputState) {
        // TODO: should we make the expand/shrink factor proportional to
        //      scoll_delta.y?
        // TODO: should we use hardcoded zoom levels as in GIMP?
        //      also note that GIMP zoom levels are not linear, but pretty arbitrary
        //      looking values
        if state.modifiers.ctrl && state.zoom_delta() > 1.0 {
            self.target_view_rect = self.target_view_rect.map(|rect| {
                // expand target view by 10%
                let expand_amount = (state.zoom_delta() - 1.0) * rect.size();
                rect.expand2(expand_amount)
            });
        } else if state.modifiers.ctrl && state.zoom_delta() < 1.0 {
            self.target_view_rect = self.target_view_rect.map(|rect| {
                // shrink target view by 10%
                let shrink_amount = (1.0 - state.zoom_delta()) * rect.size();
                rect.shrink2(shrink_amount)
            });
        }
    }

    fn add_viewport_contents(&mut self, _ui : &mut egui::Ui) {
        // empty
    }

    fn handle_viewport_response(&mut self, resp : egui::Response) {
        let resp = resp.interact(egui::Sense::drag());
        if resp.dragged_by(egui::PointerButton::Middle) {
            self.translate_target_view(resp.drag_delta());
        }
    }

    fn translate_target_view(&mut self, delta : egui::Vec2) {
        self.target_view_rect = self.target_view_rect.map(
                |rect| rect.translate(delta));
    }
}
