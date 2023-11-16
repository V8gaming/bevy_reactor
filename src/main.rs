use bevy::{prelude::*, winit::WinitSettings};
fn main() {
    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .insert_resource(ClearColor(Color::rgb_u8(255, 255, 240)))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_reactor)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup(
    mut commands: Commands,
) {

    commands.spawn(Camera2dBundle::default());


}

fn spawn_reactor(
    mut commands: Commands,
    window: Query<&Window>,
) {
    // startup neutron sources (12)
    // control rods (167)
    // short control rods from below reactor (32)
    // automatic control rods (12)
    // pressure tubes with fuel rods (1661)
    
    // postition arrays, bottom left is 0,0
    let startup_neutron_sources = [
        Vec2::new(16.0, 41.0), Vec2::new(32.0, 41.0),
        Vec2::new(8.0, 33.0), Vec2::new(24.0, 33.0), Vec2::new(40.0, 33.0),
        Vec2::new(16.0, 25.0), Vec2::new(32.0, 25.0),
        Vec2::new(8.0, 17.0), Vec2::new(24.0, 17.0), Vec2::new(40.0, 17.0),
        Vec2::new(16.0, 9.0), Vec2::new(32.0, 9.0),
        ];
    let short_control_rods = [
        Vec2::new(12.0, 45.0), Vec2::new(20.0, 45.0), Vec2::new(28.0, 45.0), Vec2::new(36.0, 45.0),
        Vec2::new(4.0, 37.0), Vec2::new(12.0, 37.0), Vec2::new(20.0, 37.0), Vec2::new(28.0, 37.0), Vec2::new(36.0, 37.0), Vec2::new(44.0, 37.0),
        Vec2::new(4.0, 29.0), Vec2::new(12.0, 29.0), Vec2::new(20.0, 29.0), Vec2::new(28.0, 29.0), Vec2::new(36.0, 29.0), Vec2::new(44.0, 29.0),
        Vec2::new(4.0, 21.0), Vec2::new(12.0, 21.0), Vec2::new(20.0, 21.0), Vec2::new(28.0, 21.0), Vec2::new(36.0, 21.0), Vec2::new(44.0, 21.0),
        Vec2::new(4.0, 13.0), Vec2::new(12.0, 13.0), Vec2::new(20.0, 13.0), Vec2::new(28.0, 13.0), Vec2::new(36.0, 13.0), Vec2::new(44.0, 13.0),
        Vec2::new(12.0, 5.0), Vec2::new(20.0, 5.0), Vec2::new(28.0, 5.0), Vec2::new(36.0, 5.0),
    ];
    let automatic_control_rods = [
        Vec2::new(24.0, 37.0),
        Vec2::new(16.0, 33.0), Vec2::new(32.0, 33.0),
        Vec2::new(24.0, 29.0),
        Vec2::new(12.0, 25.0), Vec2::new(20.0, 25.0), Vec2::new(28.0, 25.0), Vec2::new(36.0, 25.0),
        Vec2::new(24.0, 21.0),
        Vec2::new(16.0, 17.0), Vec2::new(32.0, 17.0),
        Vec2::new(24.0, 13.0),

    ];

    let control_rods = [
        // 4 space
        Vec2::new(18.0, 47.0), Vec2::new(22.0, 47.0), Vec2::new(26.0, 47.0), Vec2::new(30.0, 47.0),
        // 8 space
        Vec2::new(16.0, 45.0), Vec2::new(24.0, 45.0), Vec2::new(32.0, 45.0),
        // 4 space
        Vec2::new(10.0, 43.0), Vec2::new(14.0, 43.0), Vec2::new(18.0, 43.0), Vec2::new(22.0, 43.0), Vec2::new(26.0, 43.0), Vec2::new(30.0, 43.0), Vec2::new(34.0, 43.0), Vec2::new(38.0, 43.0),
        // 4 space ( exclude 16 & 32 )
        Vec2::new(8.0, 41.0), Vec2::new(12.0, 41.0), Vec2::new(20.0, 41.0), Vec2::new(24.0, 41.0), Vec2::new(28.0, 41.0), Vec2::new(36.0, 41.0), Vec2::new(40.0, 41.0),
        // 4 space
        Vec2::new(6.0, 39.0), Vec2::new(10.0, 39.0), Vec2::new(14.0, 39.0), Vec2::new(18.0, 39.0), Vec2::new(22.0, 39.0), Vec2::new(26.0, 39.0), Vec2::new(30.0, 39.0), Vec2::new(34.0, 39.0), Vec2::new(38.0, 39.0), Vec2::new(42.0, 39.0),
        // 8 space ( exclude 24)
        Vec2::new(8.0, 37.0), Vec2::new(16.0, 37.0), Vec2::new(32.0, 37.0), Vec2::new(40.0, 37.0),
        // 4 space
        Vec2::new(6.0, 35.0), Vec2::new(10.0, 35.0), Vec2::new(14.0, 35.0), Vec2::new(18.0, 35.0), Vec2::new(22.0, 35.0), Vec2::new(26.0, 35.0), Vec2::new(30.0, 35.0), Vec2::new(34.0, 35.0), Vec2::new(38.0, 35.0), Vec2::new(42.0, 35.0),
        // 8 space
        Vec2::new(4.0, 33.0), Vec2::new(12.0, 33.0), Vec2::new(20.0, 33.0), Vec2::new(28.0, 33.0), Vec2::new(36.0, 33.0), Vec2::new(44.0, 33.0),
        // 4 space
        Vec2::new(2.0, 31.0), Vec2::new(6.0, 31.0), Vec2::new(10.0, 31.0), Vec2::new(14.0, 31.0), Vec2::new(18.0, 31.0), Vec2::new(22.0, 31.0), Vec2::new(26.0, 31.0), Vec2::new(30.0, 31.0), Vec2::new(34.0, 31.0), Vec2::new(38.0, 31.0), Vec2::new(42.0, 31.0), Vec2::new(46.0, 31.0),
        // 8 space ( exclude 24)
        Vec2::new(8.0, 29.0), Vec2::new(16.0, 29.0), Vec2::new(32.0, 29.0), Vec2::new(40.0, 29.0),
        // 4 space
        Vec2::new(2.0, 27.0), Vec2::new(6.0, 27.0), Vec2::new(10.0, 27.0), Vec2::new(14.0, 27.0), Vec2::new(18.0, 27.0), Vec2::new(22.0, 27.0), Vec2::new(26.0, 27.0), Vec2::new(30.0, 27.0), Vec2::new(34.0, 27.0), Vec2::new(38.0, 27.0), Vec2::new(42.0, 27.0), Vec2::new(46.0, 27.0),
        // 4 space (exclude 12, 16, 20 & 32, 36, 40)
        Vec2::new(4.0, 25.0), Vec2::new(8.0, 25.0), Vec2::new(24.0, 25.0), Vec2::new(28.0, 25.0), Vec2::new(44.0, 25.0), Vec2::new(48.0, 25.0),
        // 4 space
        Vec2::new(2.0, 23.0), Vec2::new(6.0, 23.0), Vec2::new(10.0, 23.0), Vec2::new(14.0, 23.0), Vec2::new(18.0, 23.0), Vec2::new(22.0, 23.0), Vec2::new(26.0, 23.0), Vec2::new(30.0, 23.0), Vec2::new(34.0, 23.0), Vec2::new(38.0, 23.0), Vec2::new(42.0, 23.0), Vec2::new(46.0, 23.0),
        // 8 space ( exclude 24)
        Vec2::new(8.0, 21.0), Vec2::new(16.0, 21.0), Vec2::new(32.0, 21.0), Vec2::new(40.0, 21.0),
        // 4 space
        Vec2::new(2.0, 19.0), Vec2::new(6.0, 19.0), Vec2::new(10.0, 19.0), Vec2::new(14.0, 19.0), Vec2::new(18.0, 19.0), Vec2::new(22.0, 19.0), Vec2::new(26.0, 19.0), Vec2::new(30.0, 19.0), Vec2::new(34.0, 19.0), Vec2::new(38.0, 19.0), Vec2::new(42.0, 19.0), Vec2::new(46.0, 19.0),
        // 8 space
        Vec2::new(4.0, 17.0), Vec2::new(12.0, 17.0), Vec2::new(20.0, 17.0), Vec2::new(28.0, 17.0), Vec2::new(36.0, 17.0), Vec2::new(44.0, 17.0),
        // 4 space
        Vec2::new(6.0, 15.0), Vec2::new(10.0, 15.0), Vec2::new(14.0, 15.0), Vec2::new(18.0, 15.0), Vec2::new(22.0, 15.0), Vec2::new(26.0, 15.0), Vec2::new(30.0, 15.0), Vec2::new(34.0, 15.0), Vec2::new(38.0, 15.0), Vec2::new(42.0, 15.0),
        // 8 space ( exclude 24)
        Vec2::new(8.0, 13.0), Vec2::new(16.0, 13.0), Vec2::new(32.0, 13.0), Vec2::new(40.0, 13.0),
        // 4 space
        Vec2::new(6.0, 11.0), Vec2::new(10.0, 11.0), Vec2::new(14.0, 11.0), Vec2::new(18.0, 11.0), Vec2::new(22.0, 11.0), Vec2::new(26.0, 11.0), Vec2::new(30.0, 11.0), Vec2::new(34.0, 11.0), Vec2::new(38.0, 11.0), Vec2::new(42.0, 11.0),
        // 4 space ( exclude 16 & 32)
        Vec2::new(8.0, 9.0), Vec2::new(12.0, 9.0), Vec2::new(20.0, 9.0), Vec2::new(24.0, 9.0), Vec2::new(28.0, 9.0), Vec2::new(36.0, 9.0), Vec2::new(40.0, 9.0),
        // 4 space
        Vec2::new(10.0, 7.0), Vec2::new(14.0, 7.0), Vec2::new(18.0, 7.0), Vec2::new(22.0, 7.0), Vec2::new(26.0, 7.0), Vec2::new(30.0, 7.0), Vec2::new(34.0, 7.0), Vec2::new(38.0, 7.0),
        // 8 space
        Vec2::new(16.0, 5.0), Vec2::new(24.0, 5.0), Vec2::new(32.0, 5.0),
        // 4 space
        Vec2::new(18.0, 3.0), Vec2::new(22.0, 3.0), Vec2::new(26.0, 3.0), Vec2::new(30.0, 3.0), Vec2::new(34.0, 3.0)

    ];

    // total of 1884 entities
    let window_size = (window.single().physical_width() as i32, window.single().physical_height() as i32);

    let radius = (window_size.1.min(window_size.0) as f32) * 0.40;
    //let radius = 100.0;
    let center = Vec2::new(window_size.0 as f32 / 2.0, window_size.1 as f32 / 2.0);

    let square_size = 10.0;
    let grid_size = Vec2::new(window_size.0 as f32 / square_size, window_size.1 as f32 / square_size);
    for x in 0..grid_size.x as usize {
        for y in 0..grid_size.y as usize {
            let pos = Vec2::new(x as f32 * square_size, y as f32 * square_size);
            let dist = (pos - center).length();
            if dist < radius {
                let offset_pos = pos - center;
                let is_sns = startup_neutron_sources.contains(&offset_pos);
                let is_scr = short_control_rods.contains(&offset_pos);
                let is_acr = automatic_control_rods.contains(&offset_pos);
                let is_cr = control_rods.contains(&offset_pos);
                let color = if is_sns {
                    Color::rgb_u8(255, 0, 0)
                } else if is_scr {
                    Color::rgb_u8(0, 0, 255)
                } else if is_acr {
                    Color::rgb_u8(0, 255, 0)
                } else if is_cr {
                    Color::rgb_u8(255, 255, 0)
                } else {
                    Color::rgb_u8(0, 0, 0)
                };

                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(square_size, square_size)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(Vec3::new(offset_pos.x, offset_pos.y, 0.0)),
                    ..Default::default()
                });
            }
        }

    }

}