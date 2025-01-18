use crate::*;

use console::*;
use mlua::{LuaSerdeExt, *};
use notify::{recommended_watcher, Watcher};
use std::sync::{mpsc, Arc, Mutex};
use ztransform::ZBundle;

pub trait LuaChip {
    fn build(&self, lua: &mut SandyLua);
}
pub struct LuaPlugin;
impl Plugin for LuaPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SandyLua>()
            .init_resource::<EvalTable>()
            .init_resource::<CorpusPath>()
            .init_resource::<CorpusWatcherRx>()
            .init_resource::<CorpusWatcher>()
            .add_event::<ReloadCorpus>()
            .add_systems(Update,(
                    corpus_path_updated,
                    update_watcher,
                    corpus_hot_reload,
                    despawn_chromes,
                    eval_corpus,
                    sandy_runner,
                    sandy_func,
                    sandy_spawn_chrome,
            ).chain())
            // --
            ;
    }
}

#[derive(Debug, Resource, Deref, Default)]
pub struct CorpusPath(pub Option<std::path::PathBuf>);

#[derive(Debug, Default, Resource, Deref)]
pub struct CorpusWatcherRx(Option<Arc<Mutex<mpsc::Receiver<notify::Result<notify::Event>>>>>);
#[derive(Debug, Default, Resource, Deref)]
pub struct CorpusWatcher(Option<notify::RecommendedWatcher>);

#[derive(Debug, Event, Clone)]
pub struct ReloadCorpus;

#[derive(Debug, Default, Resource, Deref)]
pub struct SandyLua(pub Lua);
#[derive(Debug, Default, Resource, Deref)]
pub struct EvalTable(pub Option<mlua::Table>);

impl SandyLua {
    pub fn new() -> SandyLua {
        SandyLua(Lua::new())
            .add_chip(console::ConsoleChip)
            .add_chip(camera::CameraChip)
            .add_chip(chrome::ColorChip)
            .add_chip(chrome::MeshChip)
            .add_chip(ztransform::GeometryChip)
            .add_chip(plotter::PlotterChip)
            .add_chip(runner::RunnerChip)
            .add_chip(dance::DanceChip)
    }
    pub fn add_chip(mut self, chip: impl LuaChip) -> Self {
        chip.build(&mut self);
        self
    }
}

fn update_watcher(
    corpus_path: Res<CorpusPath>,
    mut corpus_watcher_rx: ResMut<CorpusWatcherRx>,
    mut corpus_watcher: ResMut<CorpusWatcher>,
) {
    if !corpus_path.is_changed() {
        return;
    }
    let mut dir = match &corpus_path.0 {
        Some(p) => p.clone(),
        None => return,
    };
    dir.pop();
    println!("watching : {:?}", dir);
    std::env::set_current_dir(&dir).unwrap();

    let (tx, rx) = mpsc::channel();
    let mut watcher = recommended_watcher(tx).unwrap();
    watcher
        .watch(dir.as_path(), notify::RecursiveMode::Recursive)
        .unwrap();
    *corpus_watcher = CorpusWatcher(Some(watcher));
    *corpus_watcher_rx = CorpusWatcherRx(Some(Arc::new(Mutex::new(rx))));
}

fn corpus_path_updated(corpus_path: Res<CorpusPath>, mut reload: EventWriter<ReloadCorpus>) {
    if !corpus_path.is_changed() || corpus_path.is_added() {
        return;
    }
    if let Some(p) = &corpus_path.0 {
        if p.is_file() {
            reload.send(ReloadCorpus);
        }
    }
}

fn corpus_hot_reload(corpus_watch: Res<CorpusWatcherRx>, mut reload: EventWriter<ReloadCorpus>) {
    let rx = match &corpus_watch.0 {
        None => return,
        Some(rx) => rx,
    };
    let rx = rx.lock().unwrap();

    loop {
        match rx.try_recv() {
            Ok(msg) => match msg {
                Ok(e) => {
                    if let notify::EventKind::Modify(notify::event::ModifyKind::Data(_)) = e.kind {
                        reload.send(ReloadCorpus);
                    }
                }
                Err(e) => {
                    println!("{:#?}", e);
                }
            },
            Err(mpsc::TryRecvError::Empty) => {
                break;
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                unreachable!("Unexpected closure of watcher channel")
            }
        }
    }
}

fn eval_corpus(
    mut reload: EventReader<ReloadCorpus>,
    corpus_path: Res<CorpusPath>,
    mut eval_table: ResMut<EvalTable>,
    mut lua: ResMut<SandyLua>,
) {
    if reload.read().count() == 0 {
        return;
    }
    *lua = SandyLua::new();

    let corpus_path = match &corpus_path.0 {
        Some(p) => p.clone(),
        None => return,
    };

    *eval_table = match lua.0.load(corpus_path).eval::<Table>() {
        Ok(table) => EvalTable(Some(table)),
        Err(Error::RuntimeError(s)) => {
            console_log("Runtime Error!");
            console_log(s);
            return;
        }
        Err(Error::FromLuaConversionError { from, to, message }) => {
            console_log("Conversion Error");
            console_log(format!("from : {:?}", from));
            console_log(format!("to   : {:?}", to));
            console_log(format!("msg  : {:?}", message));
            console_log("Please make sure that your lua script return a proper Dance table.");
            return;
        }
        Err(e) => {
            console_log(format!("{:#?}", e));
            return;
        }
    };
}

fn sandy_runner(
    mut reload: EventReader<ReloadCorpus>,
    table: Res<EvalTable>,
    mut runner: ResMut<runner::Runner>,
    mut event: EventWriter<runner::TickEvent>,
    lua: Res<SandyLua>,
) {
    if reload.read().count() == 0 {
        return;
    }
    if let Some(table) = &table.0 {
        *runner = lua
            .from_value(table.get("runner").unwrap_or(Value::Nil))
            .unwrap_or_default();
        runner.timer = Timer::new(
            std::time::Duration::from_millis(runner.ms_per_tick),
            TimerMode::Repeating,
        );
        event.send(runner::TickEvent(0));
    }
}

fn sandy_func(
    mut reload: EventReader<ReloadCorpus>,
    table: Res<EvalTable>,
    mut on_start: ResMut<dance::DanceOnStart>,
    mut on_tick: ResMut<dance::DanceOnTick>,
) {
    if reload.read().count() == 0 {
        return;
    }
    if let Some(table) = &table.0 {
        *on_start = dance::DanceOnStart(table.get("on_start").ok());
        *on_tick = dance::DanceOnTick(table.get("on_tick").ok());
    }
}

fn sandy_spawn_chrome(
    mut cmd: Commands,
    table: Res<EvalTable>,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    lua: Res<SandyLua>,
) {
    if !table.is_changed() {
        return;
    }

    let table = match &table.0 {
        Some(t) => t,
        None => return,
    };

    let chromes: Table = table.get("chromes").unwrap_or(lua.create_table().unwrap());

    for pair in chromes.pairs::<Value, Table>() {
        let (_, c) = pair.unwrap();

        let chr = chrome::Chrome {
            on_tick: c.get("on_tick").unwrap(),
        };

        //println!("{:?}", ZBundle::default());

        let chr = cmd
            //.spawn((chr, Visibility::Inherited, ZBundle::default()))
            .spawn((chr, Visibility::default(), ZBundle::default()))
            .id();

        let parts: Table = c.get("parts").unwrap();

        for p in parts.pairs::<Value, chrome::ChromePart>() {
            let (_, p) = p.unwrap();
            //println!("{:?}", p);

            let child = cmd
                .spawn((
                    p.offset.0,
                    Mesh3d(meshs.add(p.mesh.as_mesh())),
                    MeshMaterial3d(materials.add(p.material.as_material())),
                ))
                .id();

            cmd.entity(chr).add_child(child);
        }
    }
}

fn despawn_chromes(
    mut cmd: Commands,
    query: Query<Entity, With<chrome::Chrome>>,
    children: Query<&Children>,
    mut reload: EventReader<ReloadCorpus>,
) {
    if reload.read().count() == 0 {
        return;
    }
    //println!("despwning");
    for q in query.iter() {
        for ch in children.iter_descendants(q) {
            cmd.entity(ch).despawn();
        }
        cmd.entity(q).despawn();
    }
}
