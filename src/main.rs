use std::f32::consts::*;

use bevy::{
    asset::LoadState,
    gltf::Gltf,
    pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap},
    prelude::*,
};
use bevy_rapier3d::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    LoadingState,
    MainState,
}

fn insert_coin(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    meshes: Res<Assets<Mesh>>,
) {
    let mesh_handle = asset_server.load("coin.glb#Mesh0/Primitive0");
    if keys.just_pressed(KeyCode::Space) {
        let mesh = meshes.get(&mesh_handle).unwrap();
        let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();
        commands
            .spawn(SceneBundle {
                scene: asset_server.load("coin.glb#Scene0"),
                transform: Transform::from_xyz(0.0, 0.1, 0.0),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(collider);
    }
}

fn load_stuff(asset_server: Res<AssetServer>) {
    let coin: Handle<Gltf> = asset_server.load("coin.glb");
    let jar: Handle<Gltf> = asset_server.load("jar.glb");
    let handles = vec![coin, jar];

    match asset_server.get_group_load_state(handles.into_iter().map(|x| x.id())) {
        LoadState::Failed => {
            panic!("Some shit failed to load")
        }
        LoadState::Loaded => {
            // next state
        }
        LoadState::Loading => {
            println!("loading")
        }
        _ => {}
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins((
            DefaultPlugins,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::LoadingState), load_stuff)
        .add_systems(Startup, setup)
        .add_systems(Update, (animate_light_direction, insert_coin))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.7, 0.7, 1.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..default()
    },));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("jar.glb#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
}

fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}
