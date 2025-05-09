use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
};

mod bevy_basic_camera;

use bevy_basic_camera::{CameraController, CameraControllerPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins((
            DefaultPlugins,
            CameraControllerPlugin,
            MaterialPlugin::<GlowyMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut glowmaterials: ResMut<Assets<GlowyMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Load Texture
    let env_texture = asset_server.load("textures/stone_alley_02_1k.hdr");

    let material = glowmaterials.add(GlowyMaterial {
        env_texture: Some(env_texture),
    });

    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.1, 0.1))),
    ));

    // orb locations
    let locations = [
        Vec3::new(-0.15, 1.0, -2.0),
        Vec3::new(1.7, 1.07, -0.61),
        Vec3::new(0.21, 1.05, 1.99),
        Vec3::new(-2.16, 1.0, 0.01),
        Vec3::new(-2.2, 1.0, 2.13),
        Vec3::new(-1.06, 2.04, 1.02),
        Vec3::new(1.94, 1.02, 1.16),
        Vec3::new(0.91, 2.47, 0.83),
        Vec3::new(0.46, 2.48, -0.81),
        Vec3::new(-2.05, 0.93, -1.92),
        Vec3::new(-1.38, 2.46, -0.91),
        Vec3::new(-0.22, 3.48, 0.18),
    ];

    for location in locations {
        // spawn orbs
        commands
            .spawn((
                Mesh3d(meshes.add(Sphere::new(1.0).mesh().uv(32, 18))),
                Transform::from_translation(location),
                MeshMaterial3d(material.clone()),
            ))
            .with_children(|parent| {
                // child light
                parent.spawn((PointLight {
                    intensity: 10000.0 * 1000.0,
                    radius: 1.0,
                    color: Color::srgb(0.5, 0.1, 0.0),
                    ..default()
                },));
            });
    }

    // camera
    commands.spawn((
        (
            Camera3d::default(),
            Transform::from_xyz(8.0, 5.0, 8.0).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
        ),
        CameraController {
            orbit_mode: true,
            orbit_focus: Vec3::new(0.0, 0.5, 0.0),
            ..default()
        }
        .print_controls(),
    ));
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for GlowyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/glowy.wgsl".into()
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GlowyMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub env_texture: Option<Handle<Image>>,
}
