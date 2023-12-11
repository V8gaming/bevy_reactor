use bevy::{prelude::*, window::PrimaryWindow, winit::WinitSettings};
/* fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .init_resource::<MouseOffset>()
        .init_resource::<VecBoxData>()
        .add_systems(Startup, setup)
        .add_systems(PostStartup, update_transform)
        .add_systems(Update, box_interactions)
        
        .add_plugins(DefaultPlugins)
        .run();
}  */

/* fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut vec: ResMut<VecBoxData>,
) {
    
    commands.spawn((Camera2dBundle::default(), MainCamera));
    let grey_box = commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(256.0, 256.0)),
            color: Color::GRAY,
            ..Default::default()
        },
        ..Default::default()
    }).id();
    let box_data = BoxData {
        base_translation: Vec3::new(0.0, 0.0, 0.0),
        base_size: Vec2::new(256.0, 256.0),
        header_height: 32.0,
        entity: Some(grey_box),
    };
    vec.data.push(box_data.clone());
    create_box(&mut commands, &asset_server, "Box 1", box_data);
      
    let red_box = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(128.0, 128.0)),
            color: Color::RED,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(-256.0 - 30.0, 0.0, 0.0)),
        ..Default::default()
    };
    let red_box = commands.spawn(red_box).id();
    let box_data = BoxData {
        base_translation: Vec3::new(-256.0 - 30.0, 0.0, 0.0),
        base_size: Vec2::new(128.0, 128.0),
        header_height: 32.0,
        entity: Some(red_box),
    };
    vec.data.push(box_data.clone());

    create_box(&mut commands, &asset_server, "Box 2", box_data);
    
    }  */

#[derive(Component)]
pub(crate)struct Box {
    dragging: bool,
    entity: Entity,
    settings: TabSettings,
}

#[derive(Resource, Default)]
pub(crate) struct MouseOffset {
    x: f32,
    y: f32,
}



#[derive(Component)]
pub(crate) struct MainCamera;

pub(crate) fn box_interactions(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Sprite, &mut Box)>,
    mouse_input: Res<Input<MouseButton>>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse_offset: ResMut<MouseOffset>,
) {
    let mut window = q_windows.single_mut();
    let (camera, camera_transform) = q_camera.single();

    if let Some(mouse_position) = window.cursor_position()
    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
    .map(|ray| ray.origin.truncate()) {
        let mut cursor_icon_set = false;

        for (header_position, sprite, data) in query.iter() {
            // check if on header
            
            let sprite_half_width = sprite.custom_size.unwrap().x / 2.0;
            let sprite_half_height = sprite.custom_size.unwrap().y / 2.0;
            let close_size = sprite.custom_size.unwrap().y;
            
            let is_inside_x = mouse_position.x >= header_position.translation.x - sprite_half_width && 
            mouse_position.x <= header_position.translation.x + sprite_half_width;
            let is_inside_y = mouse_position.y >= header_position.translation.y - sprite_half_height && 
            mouse_position.y <= header_position.translation.y + sprite_half_height;

            let is_inside_close_x = 
            mouse_position.x >= header_position.translation.x + sprite_half_width - close_size &&
            mouse_position.x <= header_position.translation.x + sprite_half_width &&
            mouse_position.y >= header_position.translation.y - sprite_half_height &&
            mouse_position.y <= header_position.translation.y;

            let is_inside_close_y =
            mouse_position.x >= header_position.translation.x + sprite_half_width - close_size &&
            mouse_position.x <= header_position.translation.x + sprite_half_width &&
            mouse_position.y >= header_position.translation.y - sprite_half_height &&
            mouse_position.y <= header_position.translation.y;

            if is_inside_close_x && is_inside_close_y {
                break;

            } else if data.dragging && data.settings.draggable {
                window.cursor.icon = CursorIcon::Grabbing;
                cursor_icon_set = true;
                break; // Break out of the loop once a box is found
            } else if is_inside_x && is_inside_y && !data.dragging && data.settings.draggable{
                window.cursor.icon = CursorIcon::Grab;
                cursor_icon_set = true;
                break; // Break out of the loop once a box is found
            }

        }

        if !cursor_icon_set {
            window.cursor.icon = CursorIcon::Default;
        }
    }
    


    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(mouse_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
        {
            for (mut header_position, sprite, mut data) in query.iter_mut() {
                // check if on header
                let sprite_half_width = sprite.custom_size.unwrap().x / 2.0;
                let sprite_half_height = sprite.custom_size.unwrap().y / 2.0;
                let close_size = sprite.custom_size.unwrap().y;
                let is_inside_x = mouse_position.x >= header_position.translation.x - sprite_half_width && 
                                  mouse_position.x <= header_position.translation.x + sprite_half_width;
                let is_inside_y = mouse_position.y >= header_position.translation.y - sprite_half_height && 
                                  mouse_position.y <= header_position.translation.y + sprite_half_height;
    
                let is_inside_close_x = 
                    mouse_position.x >= header_position.translation.x + sprite_half_width - close_size &&
                    mouse_position.x <= header_position.translation.x + sprite_half_width &&
                    mouse_position.y >= header_position.translation.y - sprite_half_height &&
                    mouse_position.y <= header_position.translation.y;
                
                let is_inside_close_y =
                    mouse_position.x >= header_position.translation.x + sprite_half_width - close_size &&
                    mouse_position.x <= header_position.translation.x + sprite_half_width &&
                    mouse_position.y >= header_position.translation.y - sprite_half_height &&
                    mouse_position.y <= header_position.translation.y;
    
                if is_inside_close_x && is_inside_close_y {
                    commands.entity(data.entity).despawn_recursive();
    
                } else if is_inside_x && is_inside_y && data.settings.draggable  {
                    // Calculate the offset between the mouse and the header
                    mouse_offset.x = mouse_position.x - header_position.translation.x;
                    mouse_offset.y = mouse_position.y - header_position.translation.y;
                    header_position.translation.z += 10.0;
                    window.cursor.icon = CursorIcon::Grabbing;
                    data.dragging = true;
                }
            }
        }
    }
    
    if mouse_input.pressed(MouseButton::Left) {
        if let Some(mouse_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
        {
            for (mut header_position, _, data) in query.iter_mut() {
                if data.dragging && data.settings.draggable {
                    // Use the offset to adjust the header position
                    header_position.translation.x = mouse_position.x - mouse_offset.x;
                    header_position.translation.y = mouse_position.y - mouse_offset.y;
                }
            }
        }
    }
    
    if mouse_input.just_released(MouseButton::Left) {
        window.cursor.icon = CursorIcon::Default;
        for (mut header_position, _, mut data) in query.iter_mut() {
            data.dragging = false;
            header_position.translation.z -= 10.0;
        }
    }
}

pub(crate) fn create_box(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &TabBundle)>,
) {
    
    for (entity, data) in query.iter() {

        let header_translation = data.base_translation;
        let header_height = data.header_height;
        let size = data.base_size;
        let child_entity = entity;
        let mut header_size = Vec2::new(size.x, header_height);
        let border_size = Vec2::new(size.x + data.settings.border_width * 2.0, size.y + data.settings.border_width * 2.0);

        if data.settings.border {
            header_size = Vec2::new(border_size.x, header_height)
        };
        
        let mut entity =commands.spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(header_size),
                color: Color::WHITE,
                ..Default::default()
            },
            transform: Transform::from_translation(header_translation),
            ..Default::default()
        });
        let mut entity = entity.insert(Box {
            dragging: false,
            entity: entity.id(),
            settings: data.settings.clone(),
        })
        .with_children(|parent| {
            // Spawn header name
            parent.spawn(Text2dBundle {
                text: Text::from_section(
                    data.name.clone(),
                    TextStyle {
                        font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                        font_size: header_size.y * 0.8,
                        color: Color::BLACK,
                    },
                ),
                transform: Transform::from_translation(Vec3::new(0.0,0.0,1.0)),
                ..Default::default()
            });
            // Spawn exit button
            if data.settings.close_button {
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(header_size.y, header_size.y)),
                        ..Default::default()
                    },
                    texture: asset_server.load("icons/close.png"),
                    transform: Transform::from_translation(Vec3::new(header_size.x / 2.0 - header_size.y / 2.0, 0.0, 1.0)),
                    
                    ..Default::default()
                    }
        
                );
            }

            // Spawn main box (child sprite)
    
        });
        // Spawn border
        if data.settings.border {
            
            entity = entity.with_children(|parent| {
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(border_size),
                        color: data.settings.border_color,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(0.0, -border_size.y / 2.0 - header_height / 2.0 + data.settings.border_width, -1.0)),
                    ..Default::default()
                });
            });
        } 
        entity.add_child(child_entity);
    }

}

pub(crate) fn update_transform(
    mut query: Query<(Entity, &TabBundle)>,
    mut q_transform: Query<&mut Transform>,
) {
    for (entity, box_data) in query.iter_mut() {
        let header_height = box_data.header_height;
        let size = box_data.base_size;
        let child_sprite_entity = entity;
        if let Ok(mut transform) = q_transform.get_mut(child_sprite_entity) {
            transform.translation = Vec3::new(0.0, -size.y / 2.0 - header_height / 2.0, 0.0);
        }
    };

}

#[derive(Clone, Component)]
pub(crate) struct TabBundle {
    pub(crate) name: String,
    pub(crate) base_translation: Vec3,
    pub(crate) base_size: Vec2,
    pub(crate) header_height: f32,
    pub(crate) settings: TabSettings,
}

impl Default for TabBundle {
    fn default() -> Self {
        Self {
            name: "Tab".to_string(),
            base_translation: Vec3::new(0.0, 0.0, 0.0),
            base_size: Vec2::new(256.0, 256.0),
            header_height: 32.0,
            settings: TabSettings::default(),
        }
    }
    
}

#[derive(Clone)]
pub(crate) struct TabSettings {
    pub(crate) close_button: bool,
    pub(crate) draggable: bool,
    pub(crate) border: bool,
    pub(crate) border_color: Color,
    pub(crate) border_width: f32,
}

impl Default for TabSettings {
    fn default() -> Self {
        Self {
            close_button: true,
            draggable: true,
            border: false,
            border_color: Color::BLACK,
            border_width: 1.0,
        }
    }
}