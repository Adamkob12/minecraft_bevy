use crate::*;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

pub fn setup_light(mut commands: Commands) {
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
}

#[derive(Component)]
pub struct Sun;

#[derive(Resource)]
pub struct CycleTimer(pub Timer);

pub fn daylight_cycle(
    mut atmosphere: AtmosphereMut<Nishita>,
    mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
) {
    atmosphere.sun_position = Vec3::new(0., 0.8, 0.8);

    if let Some((mut light_trans, mut directional)) = query.single_mut().into() {
        light_trans.rotation = Quat::from_rotation_x(-2.85);
        directional.illuminance = 25000.0;
    }
}
