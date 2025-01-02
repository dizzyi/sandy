use chrome::Chrome;
use runner::RunnerEvent;
use ztransform::ZTransform;

use crate::*;

pub struct DancePlugin;

impl Plugin for DancePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DanceOnStart>()
            // --
            .add_systems(Update, dance_tick)
            .add_systems(Update, dance_on_start)
            // --
            ;
    }
}

#[derive(Clone, Debug, Resource, Default)]
pub struct DanceOnStart(pub Option<mlua::Function>);

fn dance_on_start(on_start: Res<DanceOnStart>, mut event: EventReader<RunnerEvent>) {
    for e in event.read() {
        match e {
            RunnerEvent::Restarted | RunnerEvent::Tick(0) => {
                if let Some(f) = &on_start.0 {
                    f.call::<()>(()).unwrap();
                }
            }
            _ => {}
        }
    }
}

fn dance_tick(mut query: Query<(&mut ZTransform, &Chrome)>, mut event: EventReader<RunnerEvent>) {
    for e in event.read() {
        let tick = match e {
            RunnerEvent::Tick(tick) => *tick,
            RunnerEvent::Restarted => 0,
        };
        for mut c in query.iter_mut() {
            *c.0 =
                c.1.on_tick
                    .call(tick)
                    .unwrap_or(ZTransform(Transform::IDENTITY));
        }
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
