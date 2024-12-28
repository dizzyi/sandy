use std::sync::LazyLock;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui_file::FileDialog;
use serde::{Deserialize, Serialize};
use ztransform::ZTransform;

pub mod camera;
pub mod channel;
pub mod chrome;
pub mod config;
pub mod console;
pub mod dance;
pub mod lua;
pub mod runner;
pub mod ztransform;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(config::ConfigPlugin)
        .add_plugins(camera::CameraPlugin)
        //.add_plugins(chrome::ChromePlugin)
        .add_plugins(runner::RunnerPlugin)
        .add_plugins(console::ConsolePlugin)
        .add_plugins(dance::DancePlugin)
        .add_plugins(lua::LuaPlugin)
        .add_systems(Startup, spawn_stuff)
        .add_systems(PostUpdate, coordinate_map)
        .run();
}

/// This system responsible to map ZTransform to Transform after all update
/// since bevy engine use Y-Up coordinate system,
/// and robotics use Z-Up coordinate system,
/// It'll be best to do all numerical calculation in Z-up system
/// and then translate them afterward for rendering.
fn coordinate_map(mut query: Query<(&mut Transform, &ztransform::ZTransform)>) {
    static WORLD_PREFIX: LazyLock<Transform> = LazyLock::new(|| {
        Transform::from_rotation(Quat::from_axis_angle(
            Vec3::from_array([1.0, 1.0, 1.0]).normalize(),
            120.0_f32.to_radians(),
        ))
    });
    for (mut t, z) in &mut query {
        *t = *WORLD_PREFIX * z.0;
    }
}

fn spawn_stuff(
    mut cmd: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
    });

    cmd.spawn((
        Mesh3d(meshs.add(Mesh::from(Plane3d {
            normal: Dir3::Z,
            half_size: Vec2::new(20.0, 10.0),
        }))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.5, 0.5))),
        ztransform::ZBundle::identity(),
    ));

    //let bun = (
    //    Mesh3d(meshs.add(chrome::ChromeMesh::Sphere { radius: 1.0 }.as_mesh())),
    //    MeshMaterial3d(materials.add(chrome::ChromeMaterial::default().as_material())),
    //    ZTransform(Transform::IDENTITY).0,
    //);
    //
    //cmd.spawn(bun);

    //cmd.spawn((
    //    Mesh3d(meshs.add(Mesh::from(Sphere { radius: 1.0 }))),
    //    MeshMaterial3d(materials.add(Color::srgb(0.0, 0.5, 1.0))),
    //    ztransform::ZBundle::from_xyz(0.0, 0.0, 1.0),
    //));
}
