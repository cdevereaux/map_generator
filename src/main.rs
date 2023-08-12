use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::RenderTarget,
    window::WindowRef,
};
use bevy_egui::{egui::{self, Ui}, EguiContexts, EguiPlugin};
use generators::MapGeneratorSettings;
use map::Map;

mod map;
mod generators;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EguiPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                mouse_drag,
                mouse_zoom,
                display_map.run_if(resource_changed::<Map>()),
                generation_options_ui,
            ),
        )
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    let texture_handle = asset_server.load("urizen_onebit_tileset__v1d0.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(12.0, 12.0),
        103,
        50,
        Some(Vec2::new(1.0, 1.0)),
        Some(Vec2::new(1.0, 1.0)),
    );
    let handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(SpriteAtlas {
        handle: handle.clone(),
    });

    let map_settings = MapGeneratorSettings::default();
    let mut map = Map::new(map_settings);
    commands.insert_resource(map_settings);
    commands.insert_resource(map);

    // Spawn second window
    let tools_window = commands
        .spawn((
            Window {
                title: "Tools window".to_owned(),
                ..default()
            },
            ToolsWindow,
        ))
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

fn display_map(
    mut commands: Commands,
    mut map: ResMut<Map>,
    atlas: Res<SpriteAtlas>,
    old_tiles_query: Query<Entity, With<MapTile>>,
) {
    //clear old map
    for old_tile_entity in old_tiles_query.iter() {
        commands.entity(old_tile_entity).despawn_recursive();
    }

    for x in 0..map.width {
        for y in 0..map.height {
            let sprite_index = if let Some(tile) = map.get((x, y)) {
                tile.sprite_index
            } else {
                2499
            };
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
}

fn mouse_drag(
    mouse: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    projection_query: Query<&OrthographicProjection, With<MainCamera>>,
    window_query: Query<&Window, Without<ToolsWindow>>,
) {
    if window_query.single().cursor_position().is_none() {
        return;
    }
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

fn create_egui_drag_value(ui: &mut Ui, name: &str, value: &mut usize)  {
    if ui
        .add(egui::DragValue::new(value).prefix(name))
        .changed()
    {
        //*value = (*value).clamp(1, 500);
    }
}

fn generation_options_ui(
    mut contexts: EguiContexts,
    mut map: ResMut<Map>,
    window_query: Query<Entity, With<ToolsWindow>>,
    mut settings: ResMut<MapGeneratorSettings>,
) {
    egui::CentralPanel::default().show(
        contexts.ctx_for_window_mut(window_query.get_single().unwrap()),
        |ui| {
            use MapGeneratorSettings::*;
            match &mut *settings {
                Cavern(settings) => {
                    create_egui_drag_value(ui, "Cavern Count: ", &mut settings.cavern_count);
                    create_egui_drag_value(ui, "Cavern Distance: ", &mut settings.max_cavern_dist);
                    create_egui_drag_value(ui, "Walk Count: ",&mut settings.walk_count);
                    create_egui_drag_value(ui, "Walk Length: ",&mut settings.walk_len);
                }
            }
            if ui.button("Reset").clicked() {
                map.reset();
                map.generate(*settings);
            }
        },
    );
}
