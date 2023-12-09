use bevy::{
    input::keyboard,
    prelude::*,
    winit::WinitSettings,
};
use tab_manager::BoxData;

mod structs;
mod tab_manager;
use crate::tab_manager::{box_interactions, create_box, MouseOffset, MainCamera, VecBoxData, update_transform};
use crate::structs::Rod;
fn main() {
    App::new()
        .init_resource::<MouseOffset>()
        .insert_resource(WinitSettings::desktop_app())
        .init_resource::<VecBoxData>()
        .insert_resource(ClearColor(Color::rgb_u8(255, 255, 240)))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_reactor)
        .add_systems(PostStartup, (update_transform, spawn_reactor_boxes)) // reactor
        .add_systems(Update, (keyboard_move, box_interactions)) // ui/screen
        .add_plugins(DefaultPlugins)
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 100.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            projection: OrthographicProjection {
                far: 1000.0,
                ..Default::default()
            },
            ..Default::default()
        },
        MainCamera,
        Basic2DController {},
    ));

}

#[derive(Component)]
struct Basic2DController {}
fn keyboard_move(
    mut query: Query<&mut Transform, With<Basic2DController>>,
    keyboard_input: Res<Input<keyboard::KeyCode>>,
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
}

fn spawn_reactor(
    mut commands: Commands, 
    window: Query<&Window>, 
    asset_server: Res<AssetServer>,
    mut vec: ResMut<VecBoxData>,
) {
    let reactor = RBMK1000::default();
    // total of 1884 entities
    let window_size = (
        window.single().physical_width() as i32,
        window.single().physical_height() as i32,
    );

    let radius = (window_size.1.min(window_size.0) as f32) * 0.40;
    //let radius = 100.0;
    //let center = Vec2::new(window_size.0 as f32 / 2.0, window_size.1 as f32 / 2.0);
    let mut fr_exclude = Vec::new();
    let square_size = (radius * 2.0 / reactor.size.x) * 0.75;
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
    let reactor_offset = Vec3::new(0.0, (window_size.1 as f32) / 6.0, 0.0);
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
            commands.spawn((
                SpriteBundle {
                    sprite: square.clone(),
                    transform: Transform::from_translation(
                        reactor_offset + Vec3::new(pos.x, pos.y, 0.0),
                    ),
                    ..Default::default()
                },
                
            ));
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: inner_square,
                        color: Color::BLUE,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(
                        reactor_offset + Vec3::new(pos.x, pos.y, 0.1),
                    ),
                    ..Default::default()
                },
                Rod::new(pos, rod_number, "startup neutron source".to_string()),
            ));
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
            commands.spawn((
                SpriteBundle {
                    sprite: square.clone(),

                    transform: Transform::from_translation(
                        reactor_offset + Vec3::new(pos.x, pos.y, 0.0),
                    ),
                    ..Default::default()
                },
                
            ));
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: inner_square,
                        color: Color::YELLOW,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(
                        reactor_offset + Vec3::new(pos.x, pos.y, 0.1),
                    ),
                    ..Default::default()
                },
                
                Rod::new(pos, rod_number, "short control rod".to_string()),
            ));
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
                commands.spawn((
                    SpriteBundle {
                        sprite: square.clone(),
                        transform: Transform::from_translation(
                            reactor_offset + Vec3::new(pos.x, pos.y, 0.0),
                        ),
                        ..Default::default()
                    },
                    
                ));
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: inner_square,
                            color: Color::RED,
                            ..Default::default()
                        },
                        transform: Transform::from_translation(
                            reactor_offset + Vec3::new(pos.x, pos.y, 1.0),
                        ),
                        ..Default::default()
                    },
                    
                    Rod::new(pos, rod_number, "automatic control rod".to_string()),
                ));
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
                commands.spawn((
                    SpriteBundle {
                        sprite: square.clone(),
                        transform: Transform::from_translation(
                            reactor_offset + Vec3::new(pos.x, pos.y, 0.0),
                        ),
                        ..Default::default()
                    },
                    
                ));
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            custom_size: inner_square,
                            color: Color::RED,
                            ..Default::default()
                        },
                        transform: Transform::from_translation(
                            reactor_offset + Vec3::new(pos.x, pos.y, 0.1),
                        ),
                        ..Default::default()
                    },
                    
                    Rod::new(pos, rod_number, "automatic control rod".to_string()),
                ));
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
            commands.spawn((
                SpriteBundle {
                    sprite: square.clone(),
                    transform: Transform::from_translation(
                        reactor_offset + Vec3::new(pos.x, pos.y, 0.0),
                    ),
                    ..Default::default()
                },
                
            ));
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: inner_square,
                        color: Color::GREEN,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(
                        reactor_offset + Vec3::new(pos.x, pos.y, 0.1),
                    ),
                    ..Default::default()
                },
                
                Rod::new(pos, rod_number, "control rod".to_string()),
            ));
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
            commands.spawn((
                SpriteBundle {
                    sprite: square.clone(),
                    transform: Transform::from_translation(
                        reactor_offset + Vec3::new(pos.x, pos.y, 0.0),
                    ),
                    ..Default::default()
                },
                
            ));
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: inner_square,
                        color: Color::GRAY,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(
                        reactor_offset + Vec3::new(pos.x, pos.y, 0.1),
                    ),
                    ..Default::default()
                },
                
                Rod::new(pos, rod_number, "fuel rod".to_string()),
            ));
        }

        y_counter -= 1;
    }

    let baseplate = SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(square_size * reactor.size.x *1.2, square_size * reactor.size.y *1.2 )),
            color: Color::ANTIQUE_WHITE,
            ..Default::default()
        },
        ..Default::default()
    };
    let baseplate = commands.spawn((baseplate, Reactor)).id();
    let box_data = BoxData {
        base_translation: Vec3::new(0.0, 0.0, 0.0),
        base_size: Vec2::new(square_size * reactor.size.x *1.2, square_size * reactor.size.y*1.2),
        header_height: 32.0,
        entity: Some(baseplate),
    };
    vec.data.push(box_data.clone());
    create_box(&mut commands, &asset_server, "Reactor", box_data);
    


}

fn spawn_reactor_boxes(
    mut q_parent: Query<Entity, With<Reactor>>,
    window: Query<&Window>,
    asset_server: ResMut<AssetServer>,
    mut commands: Commands
) {
    let parent = q_parent.single_mut();
    let mut parent = commands.entity(parent);
    
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
                Vec3::new(-tenth.x, tenth.y, 1.0),
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
                Vec3::new(-tenth.x, -tenth.y, 1.0),
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
                Vec3::new(tenth.x, tenth.y, 1.0),
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
                Vec3::new(tenth.x, -tenth.y, 1.0),
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
    
}

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
struct Reactor;