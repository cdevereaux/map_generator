use bevy::{prelude::*, input::mouse::{MouseMotion, MouseWheel}, render::camera::RenderTarget, window::WindowRef};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use map::Map;

mod map;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(
        ImagePlugin::default_nearest(),
    ))
    .add_plugins(EguiPlugin)
    .insert_resource(ClearColor(Color::BLACK))
    .add_systems(Startup, setup)
    .add_systems(Update, (mouse_drag, mouse_zoom, display_map.run_if(resource_changed::<Map>()), generation_options_ui))
    .run();
}

#[derive(Resource)]
pub struct SpriteAtlas {
    handle: Handle<TextureAtlas>,
}

#[derive(Component)]
pub struct MapTile;

#[derive(Component)]
pub struct ToolsWindow;

#[derive(Component)]
pub struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>,) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
    
    let texture_handle = asset_server.load("urizen_onebit_tileset__v1d0.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(12.0, 12.0), 103, 50, Some(Vec2::new(1.0, 1.0)), Some(Vec2::new(1.0, 1.0)));
    let handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(SpriteAtlas{handle: handle.clone()});

    let mut map = Map::new();
    map.generate();
    commands.insert_resource(map);

    // Spawn second window
    let tools_window = commands
        .spawn((Window {
            title: "Tools window".to_owned(),
            ..default()
        }, ToolsWindow))
        .id();

    // Spawn dummy camera for window
    commands.spawn(Camera2dBundle {
        camera: Camera {
            target: RenderTarget::Window(WindowRef::Entity(tools_window)),
            ..default()
        },
        ..default()
    });

}

fn display_map(mut commands: Commands, mut map: ResMut<Map>, atlas: Res<SpriteAtlas>, old_tiles_query: Query<Entity, With<MapTile>>) {
    if !map.reset { return; }
    
    //clear old map
    for old_tile_entity in old_tiles_query.iter() {
        commands.entity(old_tile_entity).despawn_recursive();
    }

    for x in 0..map.width {
        for y in 0..map.height {
            let sprite_index = if let Some(tile) = map.get((x, y)) { tile.sprite_index } else { 2499 };
            commands.spawn((
                SpriteSheetBundle {
                texture_atlas: atlas.handle.clone(),
                sprite: TextureAtlasSprite::new(sprite_index),
                transform: Transform {
                    translation: Vec3::new(x as f32, y as f32, 0.0) * Vec3::splat(12.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            MapTile,
        ));
        }
    }
    map.reset = false;
}

fn mouse_drag(
    mouse: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    projection_query: Query<&OrthographicProjection, With<MainCamera>>,
    window_query: Query<&Window, Without<ToolsWindow>>,
) {
    if let None = window_query.single().cursor_position() {return;}
    if mouse.pressed(MouseButton::Left) {
        let mut camera_transform = camera_query.single_mut();
        let zoom_level = projection_query.get_single().unwrap().scale;
        for event in motion_evr.iter() {
            camera_transform.translation.x -= event.delta.x * zoom_level;
            camera_transform.translation.y += event.delta.y * zoom_level;
        }
    }

}

fn mouse_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut projection_query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {

    let mut projection = projection_query.single_mut();
    for event in scroll_evr.iter() {
       let zoom_delta = match event.y {
            y if y > 0.0 => 1.0 / 1.1,
            y if y < 0.0 => 1.1,
            _ => 1.0,
        };
        projection.scale *= zoom_delta;
    }

}

fn generation_options_ui(mut contexts: EguiContexts, mut map: ResMut<Map>, window_query: Query<Entity, With<ToolsWindow>>) {
    egui::CentralPanel::default().show(contexts.ctx_for_window_mut(window_query.get_single().unwrap()), |ui| {
        if ui
            .add(egui::DragValue::new(&mut map.cavern_count).prefix("Cavern Count: "))
            .changed()
        {
            map.cavern_count = map.cavern_count.clamp(1, 64);
        }

        if ui
            .add(egui::DragValue::new(&mut map.max_cavern_dist).prefix("Max. Cavern Dist.: "))
            .changed()
        {
            map.max_cavern_dist = map.max_cavern_dist.clamp(0, 300);
        }

        if ui
            .add(egui::DragValue::new(&mut map.walk_count).prefix("Walk Count: "))
            .changed()
        {
            map.walk_count = map.walk_count.clamp(1, 100);
        }

        if ui
            .add(egui::DragValue::new(&mut map.walk_len).prefix("Walk Length: "))
            .changed()
        {
            map.walk_len = map.walk_len.clamp(1, 500);
        }

        if ui.button("Reset").clicked() {
            map.reset();
            map.generate();
        }
    });
}


// struct App {
//     map: map::Map,
//     map_texture: TextureHandle,
//     zoom: f32,
//     image_translation: Vec2,
// }

// impl App {
//     fn new(cc: &eframe::CreationContext<'_>) -> Self {
//         cc.egui_ctx.set_visuals(egui::Visuals {
//             window_shadow: Shadow::NONE,
//             ..Default::default()
//         });
//         let mut map = map::Map::new();
//         map.generate();
//         let map_texture = cc.egui_ctx.load_texture(
//             "map".to_string(),
//             ImageData::Color(map.to_color_image()),
//             TextureOptions::NEAREST,
//         );
//         Self {
//             map,
//             map_texture,
//             zoom: 1.0,
//             image_translation: Vec2::ZERO,
//         }
//     }
// }

// impl eframe::App for App {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::Window::new("Tools").title_bar(false).show(ctx, |ui| {
//             ui.label("Tools");

//             if ui
//                 .add(egui::DragValue::new(&mut self.map.cavern_count).prefix("Cavern Count: "))
//                 .changed()
//             {
//                 self.map.cavern_count = self.map.cavern_count.clamp(1, 64);
//             }

//             if ui
//                 .add(egui::DragValue::new(&mut self.map.max_cavern_dist).prefix("Max. Cavern Dist.: "))
//                 .changed()
//             {
//                 self.map.max_cavern_dist = self.map.max_cavern_dist.clamp(0, 300);
//             }

//             if ui
//                 .add(egui::DragValue::new(&mut self.map.walk_count).prefix("Walk Count: "))
//                 .changed()
//             {
//                 self.map.walk_count = self.map.walk_count.clamp(1, 100);
//             }

//             if ui
//                 .add(egui::DragValue::new(&mut self.map.walk_len).prefix("Walk Length: "))
//                 .changed()
//             {
//                 self.map.walk_len = self.map.walk_len.clamp(1, 500);
//             }

//             if ui.button("Reset").clicked() {
//                 self.map.reset();
//                 self.map.generate();
//                 self.map_texture.set(
//                     ImageData::Color(self.map.to_color_image()),
//                     TextureOptions::NEAREST,
//                 );
//             }
//         });

//         let response = egui::CentralPanel::default()
//             .show(ctx, |ui| {
//                 let image_center = ui.clip_rect().center() + self.image_translation;
//                 let image_size = Vec2 {
//                     x: self.map_texture.size_vec2().x * self.zoom,
//                     y: self.map_texture.size_vec2().y * self.zoom,
//                 };

//                 let painter = ui.painter();
//                 painter.image(
//                     self.map_texture.id(),
//                     Rect::from_center_size(image_center, image_size),
//                     Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
//                     Color32::WHITE,
//                 );

//                 let zoom_delta = ui.input(|i| match i.scroll_delta.y {
//                     x if x > 0.0 => 1.1,
//                     x if x < 0.0 => 1.0 / 1.1,
//                     _ => 1.0,
//                 });
//                 self.zoom = (self.zoom * zoom_delta).clamp(0.1, 10.0);
//             })
//             .response
//             .interact(Sense::click_and_drag());

//         let drag_delta = response.drag_delta();
//         self.image_translation += drag_delta;
//     }
// }
