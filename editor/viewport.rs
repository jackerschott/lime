pub struct Viewport {
    texture : egui::TextureHandle,
}

impl Viewport {
    pub fn new(texture : egui::TextureHandle) -> Self {
        Self { texture }
    }
}

impl egui::Widget for Viewport {
    fn ui(self, ui : &mut egui::Ui) -> eframe::egui::Response {
        ui.image(&self.texture, self.texture.size().map(|x| x as f32))
    }
}
