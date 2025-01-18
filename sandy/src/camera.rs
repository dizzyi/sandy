use std::sync::{mpsc, Arc, Mutex};

use crate::*;

use crate::ztransform::*;

pub type CameraRx = Arc<Mutex<mpsc::Receiver<CameraEvent>>>;
pub type CameraTx = Arc<mpsc::Sender<CameraEvent>>;

static CAMERA_CHANNEL: channel::LazyChannel<CameraEvent> = channel::lazy_channel!();

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraCoord>()
            .add_systems(Startup, camera_setup)
            .add_systems(Update, (camera_keyboard, gamepad_sys, camera_event))
            .add_systems(Update, camera_update);
    }
}

#[derive(Debug, Clone, PartialEq, Event, Serialize, Deserialize)]
pub enum CameraEvent {
    North(f32),
    East(f32),
    Forward(f32),
    Switch,
    Set(CameraCoord),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Resource)]
pub enum CameraCoord {
    Plane { x: f32, y: f32, z: f32 },
    Space { r: f32, rx: f32, rz: f32 },
}

impl CameraCoord {
    fn default_plane() -> CameraCoord {
        CameraCoord::Plane {
            x: CameraCoord::DEFAULT_X,
            y: CameraCoord::DEFAULT_Y,
            z: CameraCoord::DEFAULT_Z,
        }
    }

    const DEFAULT_X: f32 = 0.0;
    const DEFAULT_Y: f32 = 0.0;
    const DEFAULT_Z: f32 = 50.0;
    const DEFAULT_R: f32 = 50.0;
    const DEFAULT_RX: f32 = 45.0;
    const DEFAULT_RZ: f32 = 45.0;

    fn make_transform(&self) -> Transform {
        match self {
            CameraCoord::Plane { x, y, z } => {
                Transform::from_xyz(*x, *y, 0.0)
                    * Transform::from_xyz(0.0, 0.0, *z).looking_at(Vec3::ZERO, Vec3::Y)
            }
            CameraCoord::Space { r, rx, rz } => {
                Transform::from_rotation(Quat::from_euler(
                    EulerRot::XYZEx,
                    rx.to_radians(),
                    0.0,
                    rz.to_radians(),
                )) * Transform::from_xyz(0.0, 0.0, *r).looking_at(Vec3::ZERO, Vec3::Y)
            }
        }
    }
    fn read_event(&mut self, event: &CameraEvent) {
        match (self, event) {
            // direct set
            (s, CameraEvent::Set(camcoord)) => {
                *s = *camcoord;
            }

            // switch camera mode
            (s @ CameraCoord::Plane { .. }, CameraEvent::Switch) => {
                *s = CameraCoord::default();
            }
            (s @ CameraCoord::Space { .. }, CameraEvent::Switch) => {
                *s = CameraCoord::default_plane();
            }

            // Plane mode movement
            (CameraCoord::Plane { x, .. }, CameraEvent::East(f)) => {
                *x += f;
            }
            (CameraCoord::Plane { y, .. }, CameraEvent::North(f)) => {
                *y -= f;
            }
            (CameraCoord::Plane { z, .. }, CameraEvent::Forward(f)) => {
                *z = (*z - f).clamp(10.0, 100.0);
            }

            // Space mode movement
            (CameraCoord::Space { rz, .. }, CameraEvent::East(f)) => {
                *rz += f;
                if *rz > 360.0 {
                    *rz = 0.0;
                } else if *rz < 0.0 {
                    *rz = 360.0;
                }
            }
            (CameraCoord::Space { rx, .. }, CameraEvent::North(f)) => {
                *rx = (*rx + f).clamp(5.0, 85.0)
            }
            (CameraCoord::Space { r, .. }, CameraEvent::Forward(f)) => {
                *r = (*r - f).clamp(10.0, 100.0);
            }
        }
    }
}

impl Default for CameraCoord {
    fn default() -> Self {
        CameraCoord::Space {
            r: CameraCoord::DEFAULT_R,
            rx: CameraCoord::DEFAULT_RX,
            rz: CameraCoord::DEFAULT_RZ,
        }
    }
}

pub fn camera_setup(mut cmd: Commands, camcoord: Res<CameraCoord>) {
    cmd.spawn((
        Camera3d::default(),
        ZBundle::new(camcoord.make_transform()),
        OrthographicProjection::default_3d(),
    ));
}

pub fn camera_update(camcoord: Res<CameraCoord>, mut cam: Query<&mut ZTransform, With<Camera3d>>) {
    if !camcoord.is_changed() || camcoord.is_added() {
        return;
    }
    *cam.get_single_mut().unwrap() = ZTransform(camcoord.make_transform());
}

pub fn camera_event(mut camcoord: ResMut<CameraCoord>) {
    while let Some(e) = CAMERA_CHANNEL.read() {
        camcoord.read_event(&e);
    }
}

fn camera_keyboard(time: Res<Time>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.pressed(KeyCode::ControlLeft) {
        return;
    }
    let mut delta = time.delta_secs() * 10.0;
    if keys.pressed(KeyCode::ShiftLeft) {
        delta *= 5.0;
    }
    let camera_key_map = &[
        (KeyCode::KeyW, CameraEvent::North(-delta)),
        (KeyCode::KeyA, CameraEvent::East(-delta)),
        (KeyCode::KeyS, CameraEvent::North(delta)),
        (KeyCode::KeyD, CameraEvent::East(delta)),
        (KeyCode::KeyE, CameraEvent::Forward(delta)),
        (KeyCode::KeyQ, CameraEvent::Forward(-delta)),
    ];

    for (k, e) in camera_key_map {
        if keys.pressed(*k) {
            CAMERA_CHANNEL.send(e.clone());
        }
    }
    if keys.just_pressed(KeyCode::KeyR) && !keys.pressed(KeyCode::ControlLeft) {
        CAMERA_CHANNEL.send(CameraEvent::Switch);
    }
}

fn gamepad_sys(time: Res<Time>, gamepads: Query<(Entity, &Gamepad)>) {
    let delta = time.delta_secs() * 50.0;
    for (_, gamepad) in &gamepads {
        let right_trigger = gamepad.get(GamepadButton::RightTrigger2).unwrap();
        if right_trigger.abs() > 0.1 {
            CAMERA_CHANNEL.send(CameraEvent::Forward(right_trigger * delta));
        }
        let left_trigger = gamepad.get(GamepadButton::LeftTrigger2).unwrap();
        if left_trigger.abs() > 0.1 {
            CAMERA_CHANNEL.send(CameraEvent::Forward(-left_trigger * delta));
        }

        let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        if left_stick_x.abs() > 0.1 {
            CAMERA_CHANNEL.send(CameraEvent::East(left_stick_x * delta));
        }
        let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
        if left_stick_y.abs() > 0.1 {
            CAMERA_CHANNEL.send(CameraEvent::North(-left_stick_y * delta));
        }

        if gamepad.just_pressed(GamepadButton::LeftThumb) {
            CAMERA_CHANNEL.send(CameraEvent::Switch);
        }
    }
}

pub struct CameraChip;

impl lua::LuaChip for CameraChip {
    fn build(&self, lua: &mut lua::SandyLua) {
        let camera = lua.create_table().unwrap();

        let plane = lua
            .create_function(|_, value: mlua::Value| {
                match value {
                    mlua::Value::Table(t) => {
                        let x = t.get("x").unwrap_or(CameraCoord::DEFAULT_X);
                        let y = t.get("y").unwrap_or(CameraCoord::DEFAULT_Y);
                        let z = t.get("z").unwrap_or(CameraCoord::DEFAULT_Z);

                        CAMERA_CHANNEL.send(CameraEvent::Set(CameraCoord::Plane { x, y, z }));
                    }
                    _ => {
                        CAMERA_CHANNEL.send(CameraEvent::Set(CameraCoord::default_plane()));
                    }
                }
                Ok(())
            })
            .unwrap();
        camera.set("plane", plane).unwrap();

        let space = lua
            .create_function(|_, value: mlua::Value| {
                match value {
                    mlua::Value::Table(t) => {
                        let r = t.get("r").unwrap_or(CameraCoord::DEFAULT_R);
                        let rx = t.get("rx").unwrap_or(CameraCoord::DEFAULT_RX);
                        let rz = t.get("rz").unwrap_or(CameraCoord::DEFAULT_RZ);

                        CAMERA_CHANNEL.send(CameraEvent::Set(CameraCoord::Space { r, rx, rz }))
                    }
                    _ => {
                        CAMERA_CHANNEL.send(CameraEvent::Set(CameraCoord::default()));
                    }
                }
                Ok(())
            })
            .unwrap();
        camera.set("space", space).unwrap();

        lua.globals().set("Camera", camera).unwrap();
    }
}
