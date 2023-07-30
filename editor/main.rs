//fn main() {
//    let args : Vec<String> = std::env::args().collect();
//    let img_path = args[1].clone();
//
//    let img = image::open(img_path).unwrap();
//
//    let window = egui::Window::new("Hello World!");
//
//
//    println!("Hello World!");
//}

use eframe::egui;

fn load_image(path : &str) -> egui::ColorImage {
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
    target_texture : Option<egui::TextureHandle>,
}

impl LimeEditor {
    fn new(_cc: &eframe::CreationContext, target : egui::ColorImage) -> Self {
        LimeEditor { target, target_texture: None }
    }
}

impl eframe::App for LimeEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let target_texture : &egui::TextureHandle = self.target_texture.get_or_insert_with(|| {
            ctx.load_texture("content", self.target.clone(), Default::default())
        });

        //egui::CentralPanel::default().show(ctx, |ui| {
        //    let viewport = Viewport::new(texture);

        //    let layout = egui::Layout::centered_and_justified(
        //            egui::Direction::TopDown);
        //    ui.with_layout(layout, |ui| {
        //        ui.add(viewport);
        //    });
        //});

        let viewport_area = egui::Area::new("viewport")
            .fixed_pos(egui::pos2(100.0, 100.0));

        let add_viewport_contents = |ui : &mut egui::Ui| {
            let viewport = Viewport::new(target_texture);

            let layout = egui::Layout::centered_and_justified(
                    egui::Direction::TopDown);
            ui.with_layout(layout, |ui| {
                ui.add(viewport);
            });
        };
        viewport_area.show(ctx, add_viewport_contents);
    }
}

struct Viewport<'a> {
    texture : &'a egui::TextureHandle,
}

impl<'a> Viewport<'a> {
    fn new(texture : &'a egui::TextureHandle) -> Self {
        Self { texture }
    }
}

impl<'a> egui::Widget for Viewport<'a> {
    fn ui(self, ui : &mut egui::Ui) -> eframe::egui::Response {
        ui.image(self.texture, self.texture.size().map(|x| x as f32))
    }
}
