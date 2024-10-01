use bevy::{
    color::palettes::css::WHITE,
    pbr::{ExtendedMaterial, MaterialExtension, OpaqueRendererMethod},
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
        render_resource::{AsBindGroup, ShaderRef},
    },
};

mod bevy_basic_camera;
use bevy_basic_camera::{CameraController, CameraControllerPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((DefaultPlugins, CameraControllerPlugin))
        .add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, FlatNormalMaterial>,
        >::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut flat_materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, FlatNormalMaterial>>>,
) {
    // Cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(create_simple_cube_mesh()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: flat_materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: WHITE.into(),
                opaque_render_method: OpaqueRendererMethod::Auto,
                ..Default::default()
            },
            extension: FlatNormalMaterial {},
        }),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 6.0),
        ..default()
    });

    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(3.0, 3.0, 3.0)
                .looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
            ..default()
        })
        .insert(
            CameraController {
                orbit_mode: true,
                orbit_focus: Vec3::new(0.0, 0.5, 0.0),
                ..default()
            }
            .print_controls(),
        );
}

// https://github.com/bevyengine/bevy/blob/v0.14.2/examples/shader/extended_material.rs
impl MaterialExtension for FlatNormalMaterial {
    fn fragment_shader() -> ShaderRef {
        "flat_normal_material.wgsl".into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FlatNormalMaterial {}

fn create_simple_cube_mesh() -> Mesh {
    let vertices = vec![
        [-0.5, -0.5, -0.5], // 0: left  bottom back
        [0.5, -0.5, -0.5],  // 1: right bottom back
        [0.5, 0.5, -0.5],   // 2: right top    back
        [-0.5, 0.5, -0.5],  // 3: left  top    back
        [-0.5, -0.5, 0.5],  // 4: left  bottom front
        [0.5, -0.5, 0.5],   // 5: right bottom front
        [0.5, 0.5, 0.5],    // 6: right top    front
        [-0.5, 0.5, 0.5],   // 7: left  top    front
    ];

    let indices = vec![
        // Front face
        4, 5, 6, 4, 6, 7, // Back face
        1, 0, 3, 1, 3, 2, // Left face
        0, 4, 7, 0, 7, 3, // Right face
        5, 1, 2, 5, 2, 6, // Top face
        3, 7, 6, 3, 6, 2, // Bottom face
        4, 0, 1, 4, 1, 5,
    ];

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(Indices::U32(indices))
}
