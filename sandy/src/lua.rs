use crate::*;

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
            //.init_state::<CodeShow>()
            //.init_resource::<Corpus>()
            .init_resource::<CorpusPath>()
            .init_resource::<CorpusWatcherRx>()
            .init_resource::<CorpusWatcher>()
            .add_event::<ReloadCorpus>()
            //.add_systems(Update, (code_show).run_if(in_state(CodeShow(true))))
            //.add_systems(StateTransition, code_show_transition)
            //.add_systems(Update, (corpus_path_updated, update_watcher, corpus_hot_reload).chain())
            //.add_systems(Update, (despawn_chromes).after(update_watcher))
            //.add_systems(Update, (eval_corpus).after(despawn_chromes))
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

//#[derive(Debug, Default, Resource, Deref)]
//pub struct Corpus(std::string::String);

#[derive(Debug, Default, Resource, Deref)]
pub struct CorpusWatcherRx(Option<Arc<Mutex<mpsc::Receiver<notify::Result<notify::Event>>>>>);
#[derive(Debug, Default, Resource, Deref)]
pub struct CorpusWatcher(Option<notify::INotifyWatcher>);

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
            .add_chip(dance::DanceChip)
    }
    pub fn add_chip(mut self, chip: impl LuaChip) -> Self {
        chip.build(&mut self);
        self
    }
}

//#[derive(Debug, Clone, PartialEq, Eq, Hash, States, Default)]
//struct CodeShow(bool);

//fn code_show(mut ctx: EguiContexts, corpus: Res<Corpus>) {
//    egui::Window::new("Code")
//        .default_open(true)
//        .default_size([500.0, 500.0])
//        .default_pos([300.0, 20.0])
//        .vscroll(true)
//        .resizable(true)
//        .show(ctx.ctx_mut(), |ui| {
//            let language = "lua";
//            let theme =
//                egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
//            egui_extras::syntax_highlighting::code_view_ui(ui, &theme, &corpus.0.clone(), language);
//        });
//}

//fn code_show_transition(
//    show: Res<State<CodeShow>>,
//    mut next: ResMut<NextState<CodeShow>>,
//    keys: Res<ButtonInput<KeyCode>>,
//) {
//    if keys.just_pressed(KeyCode::KeyB) && keys.pressed(KeyCode::ControlLeft) {
//        next.set(CodeShow(!show.0));
//    }
//}

fn update_watcher(
    //mut corpus: ResMut<Corpus>,
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
    mut cmd: Commands,
    mut reload: EventReader<ReloadCorpus>,
    corpus_path: Res<CorpusPath>,
    mut eval_table: ResMut<EvalTable>,
    //mut meshs: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    mut lua: ResMut<SandyLua>,
    //mut runner: ResMut<runner::Runner>,
    //mut on_start: ResMut<dance::DanceOnStart>,
    //mut on_tick: ResMut<dance::DanceOnTick>,
    //mut event: EventWriter<runner::RunnerEvent>,
) {
    if reload.read().count() == 0 {
        return;
    } //println!("evaling");
    *lua = SandyLua::new();

    let corpus_path = match &corpus_path.0 {
        Some(p) => p.clone(),
        None => return,
    };

    *eval_table = match lua.0.load(corpus_path).eval::<Table>() {
        Ok(table) => EvalTable(Some(table)),
        Err(Error::RuntimeError(s)) => {
            console::CONSOLE_CHANNEL.send("Runtime Error!");
            console::CONSOLE_CHANNEL.send(s);
            return;
        }
        Err(Error::FromLuaConversionError { from, to, message }) => {
            console::CONSOLE_CHANNEL.send("Conversion Error");
            console::CONSOLE_CHANNEL.send(format!("from : {:?}", from));
            console::CONSOLE_CHANNEL.send(format!("to   : {:?}", to));
            console::CONSOLE_CHANNEL.send(format!("msg  : {:?}", message));
            console::CONSOLE_CHANNEL
                .send("Please make sure that your lua script return a proper Dance table.");
            return;
        }
        Err(e) => {
            console::CONSOLE_CHANNEL.send(format!("{:#?}", e));
            return;
        }
    };
}

fn sandy_runner(
    mut reload: EventReader<ReloadCorpus>,
    table: Res<EvalTable>,
    mut runner: ResMut<runner::Runner>,
    mut event: EventWriter<runner::RunnerEvent>,
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
        event.send(runner::RunnerEvent::Restarted);
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
    mut reload: EventReader<ReloadCorpus>,
    mut cmd: Commands,
    table: Res<EvalTable>,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    lua: Res<SandyLua>,
) {
    if reload.read().count() == 0 {
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
