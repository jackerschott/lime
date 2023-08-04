use eframe::egui::*;

mod viewport;

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

    let options = eframe::NativeOptions::default();
    return eframe::run_native("Lime Editor", options, Box::new(|cc| {
        let lime_editor = LimeEditor::new(cc, edit_target);
        Box::<LimeEditor>::new(lime_editor)
    }));
}

struct LimeEditor {
    target : egui::ColorImage,
    target_view : Option<egui::TextureHandle>,
}

impl LimeEditor {
    fn new(_cc: &eframe::CreationContext, target : egui::ColorImage) -> Self {
        LimeEditor { target, target_view: None }
    }
}

impl eframe::App for LimeEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let target_view = self.target_view.get_or_insert_with(|| {
            ctx.load_texture("content", self.target.clone(), Default::default())
        }).clone();

        let viewport_area = egui::Area::new("viewport")
            .fixed_pos(egui::pos2(100.0, 100.0));

        let add_viewport_contents = |ui : &mut egui::Ui| {
            let viewport = viewport::Viewport::new(target_view);

            let layout = egui::Layout::centered_and_justified(
                    egui::Direction::TopDown);
            ui.with_layout(layout, |ui| {
                let viewport_resp = ui.add(viewport);
                self.handle_viewport_response(viewport_resp);
            });
        };
        viewport_area.show(ctx, add_viewport_contents);
    }
}

impl LimeEditor {
    fn handle_viewport_response(&self, resp : Response) {
        println!("{:?}", resp.sense);
        if resp.clicked() {
            println!("Clicked!");
        }
    }
}
