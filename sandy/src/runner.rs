use crate::*;

pub struct RunnerPlugin;

impl Plugin for RunnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Runner>()
            .add_event::<RunnerEvent>()
            .init_state::<RunnerShow>()
            .add_systems(Update, runner_update)
            .add_systems(Update, (runner_show).run_if(in_state(RunnerShow(true))))
            .add_systems(StateTransition, runner_state_transition)
            //.add_systems(Update, runner_event_listen)
            // -- 
            ;
    }
}

//#[derive(Debug, Clone, States, Default, PartialEq, Eq, Hash)]
//pub enum RunnerShow {
//    #[default]
//    Show,
//    Hidden,
//}

#[derive(Debug, Clone, States, Default, PartialEq, Eq, Hash)]
pub struct RunnerShow(bool);

#[derive(Debug, Clone, Event)]
pub enum RunnerEvent {
    Tick(u64),
    Restarted,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
pub enum RunnerMode {
    Once,
    Repeat,
}

#[derive(Debug, Resource, Deserialize)]
#[serde(default)]
pub struct Runner {
    pub mode: RunnerMode,
    pub ms_per_tick: u64,
    pub tick: u64,
    pub max_tick: u64,
    pub running: bool,
    #[serde(skip)]
    pub timer: Timer,
}

impl Default for Runner {
    fn default() -> Self {
        Runner {
            mode: RunnerMode::Once,
            ms_per_tick: 10,
            tick: 0,
            max_tick: 1000,
            running: true,
            timer: Timer::new(std::time::Duration::from_millis(100), TimerMode::Repeating),
        }
    }
}

fn runner_update(time: Res<Time>, mut runner: ResMut<Runner>, mut event: EventWriter<RunnerEvent>) {
    if !runner.running {
        return;
    }
    runner.timer.tick(time.delta());
    if !runner.timer.finished() {
        return;
    }
    runner.timer.reset();
    let ms_per_tick = runner.ms_per_tick;
    runner
        .timer
        .set_duration(std::time::Duration::from_millis(ms_per_tick));

    runner.tick += 1;
    if runner.tick < runner.max_tick {
        event.send(RunnerEvent::Tick(runner.tick));
    } else if runner.mode == RunnerMode::Repeat {
        runner.tick = 0;
        event.send(RunnerEvent::Restarted);
    } else {
        runner.running = false;
    }
}

//fn runner_event_listen(mut event: EventReader<RunnerEvent>) {
//    for i in event.read() {
//        println!("{:?}", i);
//    }
//}

fn runner_state_transition(
    show: Res<State<RunnerShow>>,
    mut next: ResMut<NextState<RunnerShow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyR) && keys.pressed(KeyCode::ControlLeft) {
        next.set(RunnerShow(!show.0));
    }
}

fn runner_show(mut ctx: EguiContexts, runner: Res<Runner>) {
    egui::Window::new("Runner")
        .default_size([100.0, 250.0])
        .default_pos([1000.0, 500.0])
        .vscroll(true)
        .resizable(true)
        .show(ctx.ctx_mut(), |ui| {
            egui::Grid::new("runner_main_grid")
                .min_col_width(100.0)
                .show(ui, |ui| {
                    ui.label("mode");
                    ui.label(format!("{:?}", runner.mode));
                    ui.end_row();
                    ui.label("tick");
                    ui.label(format!("{:?}", runner.tick));
                    ui.end_row();
                    ui.label("ms_per_tick");
                    ui.label(format!("{:?}", runner.ms_per_tick));
                    ui.end_row();
                    ui.label("max_tick");
                    ui.label(format!("{:?}", runner.max_tick));
                    ui.end_row();
                    ui.label("running");
                    ui.label(format!("{:?}", runner.running));
                    ui.end_row();
                })
        });
}
