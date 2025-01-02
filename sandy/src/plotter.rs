use channel::{lazy_channel, LazyChannel};
use egui_plot::PlotMemory;
use std::collections::BTreeMap;

use crate::*;

pub struct PlotterPlugin;

impl Plugin for PlotterPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<PlotterShow>()
            .init_resource::<PlotDB>()
            .init_resource::<PlotterSelected>()
            .add_systems(StateTransition, plotter_state_transition)
            .add_systems(Update, plotter_update)
            .add_systems(Update, (plotter_show).run_if(in_state(PlotterShow(true))))
            // --
            ;
    }
}

static PLOTTER_CHANNEL: LazyChannel<PlotterMessage> = lazy_channel!();

#[derive(Debug, Clone)]
pub enum PlotterMessage {
    PushPoint(String, f64),
    ClearAll,
}

#[derive(Debug, Clone, States, Default, PartialEq, Eq, Hash)]
pub struct PlotterShow(bool);

#[derive(Debug, Resource, Default)]
pub struct PlotDB {
    pub data: BTreeMap<String, Vec<[f64; 2]>>,
}

fn plotter_update(mut db: ResMut<PlotDB>, runner: Res<runner::Runner>) {
    while let Some(msg) = PLOTTER_CHANNEL.read() {
        match msg {
            PlotterMessage::PushPoint(s, y) => {
                let x = runner.tick as f64;
                match db.data.get_mut(&s) {
                    Some(v) => v.push([x, y]),
                    None => {
                        let v = vec![[x, y]];
                        db.data.insert(s, v);
                    }
                }
            }
            PlotterMessage::ClearAll => {
                db.data.clear();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlotterChip;

impl lua::LuaChip for PlotterChip {
    fn build(&self, lua: &mut lua::SandyLua) {
        let plotter = lua.create_table().unwrap();

        let push = lua
            .create_function(|_, (s, y): (String, f64)| {
                PLOTTER_CHANNEL.send(PlotterMessage::PushPoint(s, y));
                Ok(())
            })
            .unwrap();
        plotter.set("push", push).unwrap();

        let clear = lua
            .create_function(|_, ()| {
                PLOTTER_CHANNEL.send(PlotterMessage::ClearAll);
                Ok(())
            })
            .unwrap();
        plotter.set("clear", clear).unwrap();

        lua.globals().set("Plotter", plotter).unwrap();
    }
}

#[derive(Debug, Clone, Resource, Default, PartialEq, Eq)]
struct PlotterSelected(Option<String>);

fn plotter_show(mut ctx: EguiContexts, db: Res<PlotDB>, mut selected: ResMut<PlotterSelected>) {
    if !db.data.is_empty() && selected.0.is_none() {
        *selected = PlotterSelected(db.data.first_key_value().map(|(k, _)| k).cloned());
    } else if db.data.is_empty() {
        *selected = PlotterSelected(None);
    }

    egui::Window::new("Plotter")
        .default_size([600.0, 500.0])
        .default_pos([500.0, 200.0])
        .vscroll(true)
        .resizable(true)
        .show(ctx.ctx_mut(), |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("list");
                    egui::Grid::new("plotter_left_grid")
                        .striped(true)
                        .num_columns(1)
                        .show(ui, |ui| {
                            for k in db.data.iter() {
                                ui.selectable_value(
                                    &mut *selected,
                                    PlotterSelected(Some(k.0.clone())),
                                    k.0,
                                );
                                ui.end_row();
                            }
                        })
                });
                ui.separator();

                ui.vertical(|ui| {
                    ui.label("graph");

                    if let Some(s) = &selected.0 {
                        egui_plot::Plot::new("plotter_plot")
                            .view_aspect(2.0)
                            .show(ui, |ui| {
                                if let Some(data) = db.data.get(s).cloned() {
                                    let plot_points: egui_plot::PlotPoints = data.into();
                                    let line = egui_plot::Line::new(plot_points);
                                    ui.line(line);
                                }
                            });
                    }
                });
            })
        });
}

fn plotter_state_transition(
    show: Res<State<PlotterShow>>,
    mut next: ResMut<NextState<PlotterShow>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyP) && keys.pressed(KeyCode::ControlLeft) {
        next.set(PlotterShow(!show.0));
    }
}
