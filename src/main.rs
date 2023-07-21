use eframe::{egui, epaint::{Rect, Rounding, Color32, Pos2, Shadow}};
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

#[derive(Default)]
struct App {
    map: map::Map,
    //rng: ThreadRng
}

// impl Default for App {
//     fn default() -> Self {
//         Self { 
//             rng: rand::thread_rng(),
//             ..Default::default()
//         }
//     }
// }

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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        egui::Window::new("Tools")
        .title_bar(false)
        .show(ctx, |ui| {
            ui.label("Tools");
            if ui.button("Add walk").clicked() {
                //self.map.random_walk(&mut self.rng);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            painter.rect_filled(ui.clip_rect(), Rounding::none(), Color32::GREEN);
            let Pos2 { x: width, y: height } = ui.clip_rect().max;

            for y in self.map.rows() {
                for x in self.map.cols() {
                    let rect = Rect { 
                        min: Pos2 { x: x as f32*10.0, y: y as f32*10.0 }, 
                        max: Pos2 { x: x as f32*10.0 + 10.0, y: y as f32*10.0 + 10.0 } 
                    };
                    if let Some(color) = self.map.at(x, y) {
                        painter.rect_filled(rect, Rounding::none(), color);
                    }
                    
                }
            }
            //print!("{:?}", ui.clip_rect());
            painter.rect_filled(Rect {min: Pos2{x: 0.0, y: 0.0}, max: Pos2{x: 10.0, y: 10.0}}, Rounding::none(), Color32::BLUE);
        });
    }
}