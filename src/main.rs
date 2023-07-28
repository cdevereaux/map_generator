use eframe::{
    egui::{self, Sense, TextureOptions},
    epaint::{pos2, Color32, ImageData, Rect, Shadow, TextureHandle, Vec2},
};

mod map;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Map Generator",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .unwrap();
}

struct App {
    map: map::Map,
    map_texture: TextureHandle,
    zoom: f32,
    image_translation: Vec2,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals {
            window_shadow: Shadow::NONE,
            ..Default::default()
        });
        let mut map = map::Map::new();
        map.generate();
        let map_texture = cc.egui_ctx.load_texture(
            "map".to_string(),
            ImageData::Color(map.to_color_image()),
            TextureOptions::NEAREST,
        );
        Self {
            map,
            map_texture,
            zoom: 1.0,
            image_translation: Vec2::ZERO,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Tools").title_bar(false).show(ctx, |ui| {
            ui.label("Tools");

            if ui
                .add(egui::DragValue::new(&mut self.map.cavern_count).prefix("Cavern Count: "))
                .changed()
            {
                self.map.cavern_count = self.map.cavern_count.clamp(1, 64);
            }

            if ui
                .add(egui::DragValue::new(&mut self.map.max_cavern_dist).prefix("Max. Cavern Dist.: "))
                .changed()
            {
                self.map.max_cavern_dist = self.map.max_cavern_dist.clamp(0, 300);
            }

            if ui
                .add(egui::DragValue::new(&mut self.map.walk_count).prefix("Walk Count: "))
                .changed()
            {
                self.map.walk_count = self.map.walk_count.clamp(1, 100);
            }

            if ui
                .add(egui::DragValue::new(&mut self.map.walk_len).prefix("Walk Length: "))
                .changed()
            {
                self.map.walk_len = self.map.walk_len.clamp(1, 500);
            }

            if ui.button("Reset").clicked() {
                self.map.reset();
                self.map.generate();
                self.map_texture.set(
                    ImageData::Color(self.map.to_color_image()),
                    TextureOptions::NEAREST,
                );
            }
        });

        let response = egui::CentralPanel::default()
            .show(ctx, |ui| {
                let image_center = ui.clip_rect().center() + self.image_translation;
                let image_size = Vec2 {
                    x: self.map_texture.size_vec2().x * self.zoom,
                    y: self.map_texture.size_vec2().y * self.zoom,
                };

                let painter = ui.painter();
                painter.image(
                    self.map_texture.id(),
                    Rect::from_center_size(image_center, image_size),
                    Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                    Color32::WHITE,
                );

                let zoom_delta = ui.input(|i| match i.scroll_delta.y {
                    x if x > 0.0 => 1.1,
                    x if x < 0.0 => 1.0 / 1.1,
                    _ => 1.0,
                });
                self.zoom = (self.zoom * zoom_delta).clamp(0.1, 10.0);
            })
            .response
            .interact(Sense::click_and_drag());

        let drag_delta = response.drag_delta();
        self.image_translation += drag_delta;
    }
}
