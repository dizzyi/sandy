use console::console_log;
use mlua::{LuaSerdeExt, ObjectLike};

use channel::{lazy_channel, LazyChannel};
use chrome::{Chrome, ChromeMaterial};
use runner::TickEvent;
use ztransform::{ZBundle, ZTransform};

use crate::*;

pub struct DancePlugin;

impl Plugin for DancePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DanceOnStart>()
            .init_resource::<DanceOnTick>()
            // --
            .add_systems(Update, (dance_after_image_clear,dance_on_start, dance_on_tick, dance_chrome_on_tick, dance_after_image).chain())
            // --
            ;
    }
}

#[derive(Clone, Debug, Resource, Default)]
pub struct DanceOnStart(pub Option<mlua::Function>);
#[derive(Clone, Debug, Resource, Default)]
pub struct DanceOnTick(pub Option<mlua::Function>);

fn dance_on_start(on_start: Res<DanceOnStart>, mut event: EventReader<TickEvent>) {
    for e in event.read() {
        if e.0 == 0 {
            if let Some(f) = &on_start.0 {
                f.call::<()>(()).unwrap();
            }
        }
    }
}
fn dance_on_tick(on_tick: Res<DanceOnTick>, mut event: EventReader<TickEvent>) {
    if let Some(on_tick) = &on_tick.0 {
        for e in event.read() {
            let tick = e.0;
            if let Err(e) = on_tick.call::<()>(tick) {
                console_log("on_start function error:");
                console_log(format!("{:?}", e));
            }
        }
    }
}

fn dance_chrome_on_tick(
    mut query: Query<(&mut ZTransform, &Chrome)>,
    mut event: EventReader<TickEvent>,
) {
    for e in event.read() {
        let tick = e.0;
        for mut c in query.iter_mut() {
            *c.0 = match c.1.on_tick.call(tick) {
                Ok(t) => t,
                Err(e) => {
                    println!("{:?}", e);
                    console_log("on_tick function error:");
                    console_log(format!("{:?}", e));
                    ZTransform(Transform::IDENTITY)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct AfterImage;

fn dance_after_image_clear(
    mut cmd: Commands,
    query: Query<Entity, With<AfterImage>>,
    mut event: EventReader<TickEvent>,
) {
    let mut trigger = false;
    for e in event.read() {
        if e.0 == 0 {
            trigger = true;
        }
    }
    if !trigger {
        return;
    }

    for i in query.iter() {
        cmd.entity(i).despawn_recursive();
    }
}

fn dance_after_image(
    mut cmd: Commands,
    query: Query<(Entity, &ZTransform), With<Chrome>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    part: Query<(&Transform, &Mesh3d)>,
    ch: Query<&Children>,
) {
    let mut material = None;
    while let Some(m) = DANCE_CHANNEL.read() {
        material = Some(m.0);
    }
    let material = match material {
        Some(m) => m,
        None => return,
    };

    let material = MeshMaterial3d(materials.add(material.as_material()));

    for (chrome, zt) in query.iter() {
        let ai = cmd
            .spawn((ZBundle::new(zt.0), AfterImage, Visibility::default()))
            .id();

        for c in ch.iter_descendants(chrome) {
            let (t, m) = part.get(c).unwrap();

            let ai_p = cmd.spawn(((*t), m.clone(), material.clone())).id();

            cmd.entity(ai).add_child(ai_p);
        }
    }
}

static DANCE_CHANNEL: LazyChannel<DanceMessage> = lazy_channel!();

#[derive(Clone, Debug)]
pub struct DanceMessage(ChromeMaterial);

pub struct DanceChip;

impl lua::LuaChip for DanceChip {
    fn build(&self, lua: &mut lua::SandyLua) {
        let dance = lua.create_table().unwrap();

        let after_image = lua
            .create_function(|lua, value: mlua::Value| {
                let material = lua.from_value(value).unwrap_or_default();
                DANCE_CHANNEL.send(DanceMessage(material));
                Ok(())
            })
            .unwrap();
        dance.set("after_image", after_image).unwrap();

        lua.globals().set("Dance", dance).unwrap();
    }
}

//fn chrome_despawn(
//    mut cmd: Commands,
//    query: Query<Entity, With<Chrome>>,
//    child: Query<&Children>,
//    model : Res<ChromeModel>
//) {
//    if !model.is_changed() {
//        return;
//    }
//    for q in query.iter() {
//        for ch in child.iter_descendants(q) {
//            cmd.entity(ch).despawn();
//        }
//        cmd.entity(q).despawn();
//    }
//}
//
//fn chrome_spawn(
//    mut cmd: Commands,
//    mut meshs: ResMut<Assets<Mesh>>,
//    mut materials: ResMut<Assets<StandardMaterial>>,
//    model : Res<ChromeModel>
//) {
//    if !model.is_changed() {
//        return ;
//    }
//
//
//
//}
