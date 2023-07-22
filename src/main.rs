use std::cmp::{min, max};

use eframe::{egui::{self, Sense}, epaint::{Rect, Rounding, Color32, Pos2, Shadow, Vec2}};
use rand::rngs::ThreadRng;

mod map;

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Map Generator", 
        native_options, 
        Box::new(|cc| Box::new(App::new(cc)))
    ).unwrap();
}

struct App {
    map: map::Map,
    rng: ThreadRng,
    scale: f32,
    origin: Vec2,
}

impl Default for App {
    fn default() -> Self {
        Self { 
            map: map::Map::new(),
            scale: 10.0,
            rng: rand::thread_rng(),
            origin: Vec2::ZERO,
        }
    }
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals { 
            window_shadow: Shadow::NONE,
            ..Default::default()
        });
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::Window::new("Tools")
        .title_bar(false)
        .show(ctx, |ui| {
            ui.label("Tools");
            if ui.button("Add walk").clicked() {
                self.map.random_walk(&mut self.rng);
            }
            if ui.button("Reset").clicked() {
                self.map.reset();
            }
        });

        let mut response = egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            let Pos2 { x: width, y: height } = ui.clip_rect().max;

            let zoom_delta = ui.input(|i| {
                match i.scroll_delta.y {
                    x if x > 0.0 => 1.1,
                    x if x < 0.0 => 1.0/1.1,
                    _ => 1.0,
                }
            });

            if zoom_delta != 1.0 {
                self.scale = (self.scale * zoom_delta).clamp(1.0, 100.0);
            }


            //Only draw visible rectangles
            let row_start = max((self.origin.y /self.scale) as usize, 0);
            let row_end = min(((self.origin.y + height) / self.scale) as usize + 1, self.map.height() );

            let col_start = max((self.origin.x /self.scale) as usize, 0);
            let col_end = min(((self.origin.x + width) / self.scale) as usize + 1, self.map.width() );

            for y in row_start..row_end {
                for x in col_start..col_end {
                    let top = y as f32*self.scale;
                    let left = x as f32*self.scale;

                    let rect = Rect {
                        min: Pos2 { x: left, y: top } - self.origin, 
                        max: Pos2 { x: left + self.scale, y: top + self.scale } - self.origin 
                    };
                    if let Some(color) = self.map.at(x, y) {
                        painter.rect_filled(rect, Rounding::none(), color);
                    }
                    
                }
            }
        }).response.interact(Sense::click_and_drag());

        let drag_delta = response.drag_delta();
        self.origin += -drag_delta;
    }
}