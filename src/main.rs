use std::cmp::{max, min};

use eframe::{
    egui::{self, Sense, TextureOptions},
    epaint::{Pos2, Rect, Rounding, Shadow, Vec2, pos2, Color32, TextureId, TextureManager, ImageData, TextureHandle},
};
use rand::rngs::ThreadRng;

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
    rng: ThreadRng,
    zoom: f32,
    image_translation: Vec2,
}

// impl Default for App {
//     fn default() -> Self {
//         let mut map = map::Map::new();
//         map.generate();
//         let map_texture = texture_manager.alloc(
//             "map".to_string(), 
//             ImageData::Color(map.to_color_image()), 
//             TextureOptions::LINEAR,
//         );
//         Self {
//             map,
//             map_texture,
//             scale: 10.0,
//             rng: rand::thread_rng(),
//             image_translation: Vec2::ZERO,
//         }
//     }
// }

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
            rng: rand::thread_rng(),
            image_translation: Vec2::ZERO,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Tools").title_bar(false).show(ctx, |ui| {
            ui.label("Tools");
            if ui.button("Reset").clicked() {
                self.map.reset();
                self.map.generate();
                self.map_texture.set(
                    ImageData::Color(self.map.to_color_image()), 
                    TextureOptions::NEAREST);
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
                    Color32::WHITE);


                let zoom_delta = ui.input(|i| match i.scroll_delta.y {
                    x if x > 0.0 => 1.1,
                    x if x < 0.0 => 1.0 / 1.1,
                    _ => 1.0,
                });
                self.zoom = (self.zoom * zoom_delta).clamp(0.1, 10.0);


                // //Only draw visible rectangles
                // let row_start = max((self.origin.y / self.scale) as usize, 0);
                // let row_end = min(
                //     ((self.origin.y + height) / self.scale) as usize + 1,
                //     self.map.height(),
                // );

                // let col_start = max((self.origin.x / self.scale) as usize, 0);
                // let col_end = min(
                //     ((self.origin.x + width) / self.scale) as usize + 1,
                //     self.map.width(),
                // );

                // for y in row_start..row_end {
                //     for x in col_start..col_end {
                //         let top = y as f32 * self.scale;
                //         let left = x as f32 * self.scale;

                //         let rect = Rect {
                //             min: Pos2 { x: left, y: top } - self.origin,
                //             max: Pos2 {
                //                 x: left + self.scale,
                //                 y: top + self.scale,
                //             } - self.origin,
                //         };
                //         if let Some(color) = self.map.at(x, y) {
                //             painter.rect_filled(rect, Rounding::none(), color);
                //         }
                //     }
                //}
            })
            .response
            .interact(Sense::click_and_drag());

        let drag_delta = response.drag_delta();
        self.image_translation += drag_delta;
    }
}
