use console::console_log;
use mlua::LuaSerdeExt;

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
            let on_tick = match &c.1.on_tick {
                Some(f) => f,
                None => continue,
            };
            *c.0 = match on_tick.call(tick) {
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
    query: Query<(Entity, &ZTransform, &Chrome)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    part: Query<(&Transform, &Mesh3d)>,
    ch: Query<&Children>,
    runner: Res<runner::Runner>,
    lua: Res<lua::SandyLua>,
) {
    //let material = MeshMaterial3d(materials.add(material.as_material()));
    for (id, zt, chrome) in query.iter() {
        let after_image = match &chrome.after_image {
            Some(f) => f,
            None => continue,
        };
        let material: ChromeMaterial = match after_image.call::<mlua::Value>(runner.tick) {
            Ok(m) => {
                if m.is_nil() {
                    continue;
                }
                lua.0.from_value(m).unwrap_or_default()
            }
            Err(e) => {
                println!("Error {:?}", e);
                continue;
            }
        };

        let material = MeshMaterial3d(materials.add(material.as_material()));

        let ai = cmd
            .spawn((ZBundle::new(zt.0), AfterImage, Visibility::default()))
            .id();

        for c in ch.iter_descendants(id) {
            let (t, m) = part.get(c).unwrap();

            let ai_p = cmd.spawn(((*t), m.clone(), material.clone())).id();

            cmd.entity(ai).add_child(ai_p);
        }
    }
}

static DANCE_CHANNEL: LazyChannel<DanceMessage> = lazy_channel!();

#[derive(Clone, Debug)]
pub enum DanceMessage {
    Clear,
}

pub struct DanceChip;

impl lua::LuaChip for DanceChip {
    fn build(&self, lua: &mut lua::SandyLua) {
        let dance = lua.create_table().unwrap();

        //let after_image = lua
        //    .create_function(|lua, value: mlua::Value| {
        //        let material = lua.from_value(value).unwrap_or_default();
        //        DANCE_CHANNEL.send(DanceMessage(material));
        //        Ok(())
        //    })
        //    .unwrap();
        //dance.set("after_image", after_image).unwrap();
        let clear = lua
            .create_function(|_lua, ()| {
                DANCE_CHANNEL.send(DanceMessage::Clear);
                Ok(())
            })
            .unwrap();
        dance.set("clear", clear).unwrap();

        lua.globals().set("Dance", dance).unwrap();
    }
}
