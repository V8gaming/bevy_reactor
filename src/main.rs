use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{Extent3d, TextureFormat, TextureDescriptor, TextureDimension, TextureUsages};
use bevy::render::view::RenderLayers;
use bevy::{
    input::keyboard,
    prelude::*,
    winit::WinitSettings,
};
use tab_manager::{TabBundle, TabSettings};

mod structs;
mod tab_manager;
use crate::tab_manager::{box_interactions, create_box, MouseOffset, MainCamera, update_transform};
use crate::structs::Rod;
fn main() {
    App::new()
        .init_resource::<MouseOffset>()
        .init_resource::<RBMK1000>()
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ClearColor(Color::rgb_u8(255, 255, 240)))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_reactor)
        .add_systems(Startup, camera_texture_spawn)
        .add_systems(PostStartup, (update_transform, create_box, spawn_reactor_boxes)) // reactor
        .add_systems(Update, (debug_keyboard, box_interactions)) // ui/screen
        .add_systems(Update,spawn_reactor_core)
        .add_plugins(DefaultPlugins)
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 1,
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            projection: OrthographicProjection {
                far: 1000.0,
                ..Default::default()
            },
            ..Default::default()
        },
        RenderLayers::all(),
        MainCamera,
        Basic2DController {},
    ));

}

#[derive(Component)]
struct Basic2DController {}
fn debug_keyboard(
    mut query: Query<&mut Transform, With<Basic2DController>>,
    keyboard_input: Res<Input<keyboard::KeyCode>>,
    mut commands: Commands,
    mut q_reactor_baseplate: Query<&mut ReactorBaseplate>,
    q_reactor: Query<Entity, With<Reactor>>,
) {
    for mut transform in query.iter_mut() {
        if keyboard_input.pressed(keyboard::KeyCode::W) {
            transform.translation.y += 1.0;
        }
        if keyboard_input.pressed(keyboard::KeyCode::S) {
            transform.translation.y -= 1.0;
        }
        if keyboard_input.pressed(keyboard::KeyCode::A) {
            transform.translation.x -= 1.0;
        }
        if keyboard_input.pressed(keyboard::KeyCode::D) {
            transform.translation.x += 1.0;
        }
    }
    if let Ok(mut reactor) = q_reactor_baseplate.get_single_mut() {

        if keyboard_input.just_pressed(keyboard::KeyCode::Q) {
            reactor.radius_multiplier += 0.1;
            reactor.changed = true;
            for entity in q_reactor.iter() {
                commands.entity(entity).despawn_recursive();
            }
        } else if keyboard_input.just_pressed(keyboard::KeyCode::E) {
            reactor.radius_multiplier -= 0.1;
            reactor.changed = true;
            for entity in q_reactor.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    };

}

fn spawn_reactor(
    mut commands: Commands, 
    window: Query<&Window>, 
    reactor: Res<RBMK1000>,
) {
    let window_size = Vec2::new(
        window.single().physical_width() as f32,
        window.single().physical_height() as f32,
    );
    // total of 1884 entities

    let radius = window_size.y.min(window_size.x) * 0.40;
    //let radius = 100.0;
    //let center = Vec2::new(window_size.0 as f32 / 2.0, window_size.1 as f32 / 2.0);
    let square_size = (radius * 2.0 / reactor.size.x) * 0.75;

    
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(square_size * reactor.size.x *1.2, square_size * reactor.size.y *1.5 )),
                color: Color::ANTIQUE_WHITE,
                ..Default::default()
            },
            ..Default::default()
        }, 
        ReactorBaseplate {
            radius_multiplier: 1.65,       
            changed: true, 
        }, 
        TabBundle {
            name: "Reactor".to_string(),
            base_translation: Vec3::new(0.0, 0.0, 0.0),
            base_size: Vec2::new(square_size * reactor.size.x *1.2, square_size * reactor.size.y*1.5),
            header_height: 32.0,
            ..Default::default()
        } 
    ));

}

fn camera_texture_spawn(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,

) {
    let scale = 4.0;
    let v2_size = Vec2::new(1366.0, 768.0) /scale;
    let size = Extent3d {
        width: v2_size.x as u32,
        height: v2_size.y as u32,
        ..default()
    };
    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    
    // fill image.data with zeroes
    image.resize(size);
    let image = images.add(image);


    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 2,
            target: RenderTarget::Image(image.clone()),
            ..Default::default()
        },
        
        projection: OrthographicProjection {
            far: 1000.0,
            scale,
            ..Default::default()
        },
        
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });


    commands.spawn((SpriteBundle {
        sprite: Sprite{
            custom_size: Some(v2_size),
            ..Default::default()
        },
        texture: image,
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    },
    
    TabBundle {
        name: "Camera".to_string(),
        base_translation: Vec3::new(256.0, 10.0, 0.0),
        base_size: v2_size,
        header_height: 32.0,
        settings: TabSettings{
            border: true,
            border_width: 10.0,
            ..Default::default()
        },
    }
    ));
    
}


fn spawn_reactor_core (
    mut q_parent: Query<(Entity, &Sprite, &mut ReactorBaseplate)>,
    mut commands: Commands,
    reactor: Res<RBMK1000>,
) {
    
    if let Ok((parent, sprite, mut reactor_data)) = q_parent.get_single_mut() {
        if !reactor_data.changed {
            return;
        }
        let mut parent = commands.entity(parent);
        let parent_size = sprite.custom_size.unwrap();
        let radius = parent_size.x / reactor_data.radius_multiplier;
        let y_offset = parent_size.y / 5.0;
        //let radius = 100.0;
        //let center = Vec2::new(window_size.0 as f32 / 2.0, window_size.1 as f32 / 2.0);
        let mut fr_exclude = Vec::new();
        let square_size = (radius * 2.0 / reactor.size.x) / reactor_data.radius_multiplier;
        reactor_data.changed = false;
        let mut rod_number = 0;
        // startup neutron sources
        let sns_inital = reactor.startup_neutron_sources.0;
        let sns_spacing = reactor.startup_neutron_sources.1;
        let sns = reactor.startup_neutron_sources.2.clone();
        let mut y_counter = sns_inital;
    
        let inner_square = Some(Vec2::new(square_size, square_size) * 0.9);
        let square = Sprite {
            custom_size: Some(Vec2::new(square_size, square_size)),
            color: Color::BLACK,
            ..Default::default()
        };
        for (mut amount, offset, spacing, exclude) in sns {
            if exclude.is_some() {
                amount += exclude.clone().unwrap().len() as i32;
            }
            let iter = offset..(offset + amount * spacing);
            for x_coord in iter.step_by(spacing as usize) {
                if exclude.is_some() && exclude.clone().unwrap().contains(&x_coord) {
                    continue;
                }
                let pos = Vec2::new(x_coord as f32, y_counter as f32);
                let pos = pos * square_size - reactor.center * square_size;
                rod_number += 1;
                parent.with_children(|parent| {
                    parent.spawn((SpriteBundle {
                        sprite: square.clone(),
                        transform: Transform::from_translation(
                            Vec3::new(pos.x, y_offset + pos.y, 1.0),
                        ),
                        ..Default::default()
                    }, Reactor));
                    parent.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: inner_square,
                                color: Color::BLUE,
                                ..Default::default()
                            },
                            transform: Transform::from_translation(
                                Vec3::new(pos.x, y_offset + pos.y, 2.0),
                            ),
                            ..Default::default()
                        },
                        Rod::new(pos, rod_number, "startup neutron source".to_string()),
                        Reactor,
                        
                    ));
                });
    
                fr_exclude.push(Vec2::new(x_coord as f32, y_counter as f32));
            }
    
            y_counter -= sns_spacing;
        }
    
        // short control rods
        let scr_inital = reactor.short_control_rods.0;
        let scr_spacing = reactor.short_control_rods.1;
        let scr = reactor.short_control_rods.2.clone();
        let mut y_counter = scr_inital;
        for (mut amount, offset, spacing, exclude) in scr {
            if exclude.is_some() {
                amount += exclude.clone().unwrap().len() as i32;
            }
            let iter = offset..(offset + amount * spacing);
            for x_coord in iter.step_by(spacing as usize) {
                if exclude.is_some() && exclude.clone().unwrap().contains(&x_coord) {
                    continue;
                }
                let pos = Vec2::new(x_coord as f32, y_counter as f32);
                let pos = pos * square_size - reactor.center * square_size;
                rod_number += 1;
                parent.with_children(|parent| {
                    parent.spawn((SpriteBundle {
                        sprite: square.clone(),
    
                        transform: Transform::from_translation(
                            Vec3::new(pos.x,y_offset + pos.y,1.0),
                        ),
                        ..Default::default()
                    },Reactor,));
                    parent.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: inner_square,
                                color: Color::YELLOW,
                                ..Default::default()
                            },
                            transform: Transform::from_translation(
                                Vec3::new(pos.x, y_offset + pos.y, 2.0),
                            ),
                            ..Default::default()
                        },
                        Rod::new(pos, rod_number, "short control rod".to_string()),
                        Reactor,
                        
                    ));
                });
                fr_exclude.push(Vec2::new(x_coord as f32, y_counter as f32));
            }
    
            y_counter -= scr_spacing;
        }
    
        // automatic control rods
        let acr_inital = reactor.automatic_control_rods.0;
        let acr_spacing = reactor.automatic_control_rods.1;
        let acr = reactor.automatic_control_rods.2.clone();
        let mut y_counter = acr_inital;
        for (mut amount, offset, spacing, exclude) in acr {
            if amount == 1 {
                let x_coord = offset;
                if !(exclude.is_some() && exclude.clone().unwrap().contains(&x_coord)) {
                    let pos = Vec2::new(x_coord as f32, y_counter as f32);
                    let pos = pos * square_size - reactor.center * square_size;
                    rod_number += 1;
                    parent.with_children(|parent| {
                        parent.spawn((SpriteBundle {
                            sprite: square.clone(),
                            transform: Transform::from_translation(
                                Vec3::new(pos.x,y_offset + pos.y,1.0),
                            ),
                            ..Default::default()
                        }, Reactor,));
                        parent.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: inner_square,
                                    color: Color::RED,
                                    ..Default::default()
                                },
                                transform: Transform::from_translation(
                                    Vec3::new(pos.x, y_offset + pos.y, 2.0),
                                ),
                                ..Default::default()
                            },
                            Rod::new(pos, rod_number, "automatic control rod".to_string()),
                            Reactor,
                        ));
    
                    });
    
                }
                fr_exclude.push(Vec2::new(x_coord as f32, y_counter as f32));
            } else {
                if exclude.is_some() {
                    amount += exclude.clone().unwrap().len() as i32;
                }
                let iter = offset..(offset + amount * spacing);
                for x_coord in iter.step_by(spacing as usize) {
                    if exclude.is_some() && exclude.clone().unwrap().contains(&x_coord) {
                        continue;
                    }
                    let pos = Vec2::new(x_coord as f32, y_counter as f32);
                    let pos = pos * square_size - reactor.center * square_size;
                    rod_number += 1;
                    parent.with_children(|parent| {
                        parent.spawn((SpriteBundle {
                            sprite: square.clone(),
                            transform: Transform::from_translation(
                                Vec3::new(pos.x,y_offset + pos.y,1.0),
                            ),
                            ..Default::default()
                        }, Reactor,));
                        parent.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    custom_size: inner_square,
                                    color: Color::RED,
                                    ..Default::default()
                                },
                                transform: Transform::from_translation(
                                    Vec3::new(pos.x, y_offset + pos.y, 2.0),
                                ),
                                ..Default::default()
                            },
                            Rod::new(pos, rod_number, "automatic control rod".to_string()),
                            Reactor,
                        ));
    
                    });
                    fr_exclude.push(Vec2::new(x_coord as f32, y_counter as f32));
                }
            }
    
            y_counter -= acr_spacing;
        }
    
        // control rods
        let cr_inital = reactor.control_rods.0;
        let cr_spacing = reactor.control_rods.1;
        let cr = reactor.control_rods.2.clone();
        let mut y_counter = cr_inital;
        for (mut amount, offset, spacing, exclude) in cr {
            if exclude.is_some() {
                amount += exclude.clone().unwrap().len() as i32;
            }
            let iter = offset..(offset + amount * spacing);
            for x_coord in iter.step_by(spacing as usize) {
                if exclude.is_some() && exclude.clone().unwrap().contains(&x_coord) {
                    continue;
                }
                let pos = Vec2::new(x_coord as f32, y_counter as f32);
                let pos = pos * square_size - reactor.center * square_size;
                rod_number += 1;
                parent.with_children(|parent| {
                    parent.spawn((SpriteBundle {
                        sprite: square.clone(),
                        transform: Transform::from_translation(
                            Vec3::new(pos.x,y_offset + pos.y,1.0),
                        ),
                        ..Default::default()
                    }, Reactor,));
                    parent.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: inner_square,
                                color: Color::GREEN,
                                ..Default::default()
                            },
                            transform: Transform::from_translation(
                                Vec3::new(pos.x, y_offset + pos.y, 2.0),
                            ),
                            ..Default::default()
                        },
                        Rod::new(pos, rod_number, "control rod".to_string()),
                        Reactor,
                    ));
                });
                fr_exclude.push(Vec2::new(x_coord as f32, y_counter as f32));
            }
    
            y_counter -= cr_spacing;
        }
    
        // fuel rods
        let fr = reactor.fuel_rods.clone();
        let mut y_counter = reactor.size.y as i32;
        for (start, end) in fr.clone().iter() {
            let iter = *start..*end;
            for x_coord in iter {
                let pos = Vec2::new(x_coord as f32, y_counter as f32);
                if fr_exclude.contains(&pos) {
                    continue;
                }
                let pos = pos * square_size - reactor.center * square_size;
                rod_number += 1;
                parent.with_children(|parent| {
                    parent.spawn((SpriteBundle {
                        sprite: square.clone(),
                        transform: Transform::from_translation(
                            Vec3::new(pos.x,y_offset + pos.y,1.0),
                        ),
                        ..Default::default()
                    }, Reactor,));
                    parent.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                custom_size: inner_square,
                                color: Color::GRAY,
                                ..Default::default()
                            },
                            transform: Transform::from_translation(
                                Vec3::new(pos.x, y_offset + pos.y, 2.0),
                            ),
                            ..Default::default()
                        },
                        Rod::new(pos, rod_number, "fuel rod".to_string()),
                        Reactor,
                    ));
                });
            }
    
            y_counter -= 1;
        }
    };

    
}

fn spawn_reactor_boxes(
    mut q_parent: Query<(Entity, &Sprite), With<ReactorBaseplate>>,
    window: Query<&Window>,
    asset_server: ResMut<AssetServer>,
    mut commands: Commands
) {
    if let Ok((parent, sprite)) = q_parent.get_single_mut() {
        let mut parent = commands.entity(parent);
        let parent_size = sprite.custom_size.unwrap();
        
        let y_offset = parent_size.y / 3.0;
        let window_size = Vec2::new(
            window.single().physical_width() as f32,
            window.single().physical_height() as f32,
        );
        let tenth = window_size / 10.0;
    
        let inner_rect = Sprite {
            custom_size: Some(Vec2::new(95.0, 45.0)),
            color: Color::WHITE,
            ..Default::default()
        };
        let rect = Sprite {
            custom_size: Some(Vec2::new(100.0, 50.0)),
            color: Color::BLACK,
            ..Default::default()
        };
        // Neutron Rate
        parent.with_children(|parent| {
            let mut entity = parent.spawn(SpriteBundle {
                sprite: rect.clone(),
                transform: Transform::from_translation(
                    Vec3::new(-tenth.x, tenth.y - y_offset, 1.0),
                ),
                ..Default::default()
            },);
            entity.with_children(|parent| {
                parent.spawn(Text2dBundle {
                    text: Text::from_section(
                        "Neutron Rate",
                        TextStyle {
                            font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    ),
                    transform: Transform::from_translation(
                        Vec3::new(0.0, 50.0, 1.0),
                    ),
                    ..Default::default()
                });
                parent.spawn((
                    SpriteBundle {
                        sprite: inner_rect.clone(),
                        transform: Transform::from_translation(
                            Vec3::new(0.0, 0.0, 2.0),
                        ),
                        ..Default::default()
                    },
                    
                ));
                parent.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            "0%",
                            TextStyle {
                                font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::RED,
                            },
                        ),
                        transform: Transform::from_translation(
                            Vec3::new(0.0, 0.0, 3.0),
                        ),
                        ..Default::default()
                    },
                    
                    NeutronRate { value: 0.0 },
                ));
                
            });
    
        });
    
        // Thermal Power
        parent.with_children(|parent| {
            let mut entity = parent.spawn(SpriteBundle {
                sprite: rect.clone(),
                transform: Transform::from_translation(
                    Vec3::new(-tenth.x, -tenth.y - y_offset, 1.0),
                ),
                ..Default::default()
            },);
            entity.with_children(|parent| {
                parent.spawn(Text2dBundle {
                    text: Text::from_section(
                        "Thermal Power",
                        TextStyle {
                            font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    ),
                    transform: Transform::from_translation(
                        Vec3::new(0.0, 50.0, 1.0),
                    ),
                    ..Default::default()
                });
                parent.spawn((
                    SpriteBundle {
                        sprite: inner_rect.clone(),
                        transform: Transform::from_translation(
                            Vec3::new(0.0, 0.0, 2.0),
                        ),
                        ..Default::default()
                    },
                    
                ));
                parent.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            "0%",
                            TextStyle {
                                font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::RED,
                            },
                        ),
                        transform: Transform::from_translation(
                            Vec3::new(0.0, 0.0, 3.0),
                        ),
                        ..Default::default()
                    },
                    
                    ThermalPower { value: 0.0 },
                ));
            });
    
        });
    
        // Neutron Flux
        parent.with_children(|parent| {
            let mut entity = parent.spawn(SpriteBundle {
                sprite: rect.clone(),
                transform: Transform::from_translation(
                    Vec3::new(tenth.x, tenth.y - y_offset, 1.0),
                ),
                ..Default::default()
            },);
            entity.with_children(|parent| {
                parent.spawn(Text2dBundle {
                    text: Text::from_section(
                        "Neutron Flux",
                        TextStyle {
                            font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    ),
                    transform: Transform::from_translation(
                        Vec3::new(0.0, 50.0, 1.0),
                    ),
                    ..Default::default()
                });
                parent.spawn((
                    SpriteBundle {
                        sprite: inner_rect.clone(),
                        transform: Transform::from_translation(
                            Vec3::new(0.0, 0.0, 2.0),
                        ),
                        ..Default::default()
                    },
                    
                ));
                parent.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            "0%",
                            TextStyle {
                                font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::RED,
                            },
                        ),
                        transform: Transform::from_translation(
                            Vec3::new(0.0, 0.0, 3.0),
                        ),
                        ..Default::default()
                    },
                    
                    NeutronFlux { value: 0.0 },
                ));
            });
    
        });
    
        // Remaining Fuel
        parent.with_children(|parent| {
            let mut entity = parent.spawn(SpriteBundle {
                sprite: rect.clone(),
                transform: Transform::from_translation(
                    Vec3::new(tenth.x, -tenth.y - y_offset, 1.0),
                ),
                ..Default::default()
            },);
            entity.with_children(|parent| {
                parent.spawn(Text2dBundle {
                    text: Text::from_section(
                        "Remaining Fuel",
                        TextStyle {
                            font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::BLACK,
                        },
                    ),
                    transform: Transform::from_translation(
                        Vec3::new(0.0, 50.0, 1.0),
                    ),
                    ..Default::default()
                });
                parent.spawn((
                    SpriteBundle {
                        sprite: inner_rect.clone(),
                        transform: Transform::from_translation(
                            Vec3::new(0.0, 0.0, 2.0),
                        ),
                        ..Default::default()
                    },
                    
                ));
                parent.spawn((
                    Text2dBundle {
                        text: Text::from_section(
                            "0%",
                            TextStyle {
                                font: asset_server.load("fonts/FiraCode-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::RED,
                            },
                        ),
                        transform: Transform::from_translation(
                            Vec3::new(0.0, 0.0, 3.0),
                        ),
                        ..Default::default()
                    },
                    
                    RemainingFuel { value: 0.0 },
                ));
            });
    
        });
    };

    
}

#[derive(Resource)]
struct RBMK1000 {
    // startup neutron sources (12)
    // control rods (167)
    // short control rods from below reactor (32)
    // automatic control rods (12)
    // pressure tubes with fuel rods (1661)
    // total 1884 entities

    // postition arrays, bottom left is 0,0
    size: Vec2,
    center: Vec2,
    startup_neutron_sources: (i32, i32, Vec<Patten>),
    short_control_rods: (i32, i32, Vec<Patten>),
    automatic_control_rods: (i32, i32, Vec<Patten>),
    control_rods: (i32, i32, Vec<Patten>), // starting y, vertical offset, row pattens
    fuel_rods: Vec<FuelPatten>,
}
// patten is amount, offset, spacing, exclude
type Patten = (i32, i32, i32, Option<Vec<i32>>);
// fuel patten is start, end
type FuelPatten = (i32, i32);
impl Default for RBMK1000 {
    fn default() -> Self {
        RBMK1000 {
            size: Vec2::new(48.0, 48.0),
            center: Vec2::new(24.0, 25.0),

            startup_neutron_sources: (
                41,
                8,
                vec![
                    // amount, offset, spacing, excluded
                    (2, 16, 16, None),
                    (3, 8, 16, None),
                    (2, 16, 16, None),
                    (3, 8, 16, None),
                    (2, 16, 16, None),
                ],
            ),

            short_control_rods: (
                45,
                8,
                vec![
                    // amount, offset, spacing, excluded
                    (4, 12, 8, None),
                    (6, 4, 8, None),
                    (6, 4, 8, None),
                    (6, 4, 8, None),
                    (6, 4, 8, None),
                    (4, 12, 8, None),
                ],
            ),
            automatic_control_rods: (
                37,
                4,
                vec![
                    // amount, offset, spacing, excluded
                    (1, 24, 0, None),
                    (2, 16, 16, None),
                    (1, 24, 0, None),
                    (4, 12, 8, None),
                    (1, 24, 0, None),
                    (2, 16, 16, None),
                    (1, 24, 0, None),
                ],
            ),
            control_rods: (
                47,
                2,
                vec![
                    // amount, offset, spacing, excluded
                    (4, 18, 4, None),
                    (3, 16, 8, None),
                    (8, 10, 4, None),
                    (7, 8, 4, Some(vec![16, 32])),
                    (10, 6, 4, None),
                    (4, 8, 8, Some(vec![24])),
                    (10, 6, 4, None),
                    (6, 4, 8, None),
                    (12, 2, 4, None),
                    (4, 8, 8, Some(vec![24])),
                    (12, 2, 4, None),
                    (5, 4, 4, Some(vec![12, 16, 20, 28, 32, 36])),
                    (12, 2, 4, None),
                    (4, 8, 8, Some(vec![24])),
                    (12, 2, 4, None),
                    (6, 4, 8, None),
                    (10, 6, 4, None),
                    (4, 8, 8, Some(vec![24])),
                    (10, 6, 4, None),
                    (7, 8, 4, Some(vec![16, 32])),
                    (8, 10, 4, None),
                    (3, 16, 8, None),
                    (5, 18, 4, None),
                ],
            ),
            fuel_rods: vec![
                // start end
                (18, 31),
                (15, 34),
                (13, 36),
                (11, 39),
                (10, 40),
                (9, 41),
                (8, 42),
                (7, 43),
                (6, 44),
                (5, 45),
                (4, 46),
                (4, 46),
                (3, 47),
                (3, 47),
                (2, 48),
                (2, 48),
                (2, 48),
                (1, 49),
                (1, 49),
                (1, 49),
                (1, 49),
                (1, 49),
                (1, 49),
                (1, 49),
                (1, 49),
                (1, 49),
                (1, 49),
                (1, 49),
                (1, 49),
                (1, 49),
                (1, 49),
                (2, 48),
                (2, 48),
                (2, 48),
                (3, 47),
                (3, 47),
                (4, 46),
                (4, 46),
                (5, 45),
                (6, 44),
                (7, 43),
                (8, 42),
                (9, 41),
                (10, 40),
                (11, 39),
                (13, 37),
                (15, 35),
                (18, 32),
            ],
        }
    }
}

#[derive(Component)]
struct NeutronRate {
    value: f32,
}
#[derive(Component)]
struct ThermalPower {
    value: f32,
}
#[derive(Component)]
struct NeutronFlux {
    value: f32,
}
#[derive(Component)]
struct RemainingFuel {
    value: f32,
}


#[derive(Component)]
struct ReactorBaseplate {
    radius_multiplier: f32,
    changed: bool,
}

#[derive(Component)]
struct Reactor;
