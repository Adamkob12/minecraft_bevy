use crate::*;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

pub fn setup_light(mut commands: Commands, primary_window: Query<&Window, With<PrimaryWindow>>) {
    // commands.insert_resource(AtmosphereModel::new(Nishita {
    //     sun_position: Vec3::new(0., 1., -1.),
    //     sun_intensity: 10.0,
    //     ..default()
    // }));
    //
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..Default::default()
            },
            ..Default::default()
        },
        Sun,
    ));
    let mut window_width = CROSSHAIR_SIZE;
    let mut window_height = CROSSHAIR_SIZE;
    if let Ok(window) = primary_window.get_single() {
        (window_width, window_height) = (window.resolution.width(), window.resolution.height());
    } else {
        warn!("Primary window not found ");
    }
    commands.spawn(
        TextBundle::from_section(
            format!("+"),
            TextStyle {
                font_size: CROSSHAIR_SIZE,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            top: Val::Px(window_height / 2.0 - CROSSHAIR_SIZE / 2.0),
            left: Val::Px(window_width / 2.0 - CROSSHAIR_SIZE / 2.0),
            ..default()
        }),
    );
}

#[derive(Component)]
pub struct Sun;

#[derive(Resource)]
pub struct CycleTimer(pub Timer);

pub fn daylight_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
) {
    atmosphere.sun_position = Vec3::new(0.0, 0.9, 0.7);

    if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
        let t = Transform::from_xyz(0.0, 0.0, 0.0).looking_to(Vec3::new(0.6, -1.0, 0.6), Vec3::Y);
        light_trans.rotation = t.rotation;
        directional.illuminance = 9000.0;
    }
}
