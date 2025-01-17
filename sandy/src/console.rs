use crate::*;
use bevy::reflect::List;
use lua::{LuaChip, SandyLua};

pub struct ConsolePlugin;

#[derive(Debug, Clone)]
pub enum ConsoleMsg {
    Log(String),
    Clear,
}

pub fn console_log(msg: impl Into<String>) {
    CONSOLE_CHANNEL.send(ConsoleMsg::Log(msg.into()));
}
pub fn console_clear() {
    CONSOLE_CHANNEL.send(ConsoleMsg::Clear);
}

static CONSOLE_CHANNEL: channel::LazyChannel<ConsoleMsg> = channel::lazy_channel!();

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Console>()
            .init_state::<ConsoleShow>()
            .add_systems(Update, console_update)
            .add_systems(Update, (console_show).run_if(in_state(ConsoleShow(true))));
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
struct ConsoleShow(bool);
impl Default for ConsoleShow {
    fn default() -> Self {
        ConsoleShow(true)
    }
}
#[derive(Debug, Resource, Default, Deref)]
pub struct Console {
    entry: std::collections::VecDeque<String>,
}

pub struct ConsoleChip;
impl LuaChip for ConsoleChip {
    fn build(&self, lua: &mut SandyLua) {
        let console = lua.create_table().unwrap();

        let print = lua
            .create_function(move |_, msg: mlua::Value| {
                console_log(
                    msg.to_string()
                        .unwrap_or("cannot convert value to string".to_string()),
                );
                Ok(())
            })
            .unwrap();
        console.set("print", print).unwrap();
        let clear = lua
            .create_function(|_, ()| {
                console_clear();
                Ok(())
            })
            .unwrap();
        console.set("clear", clear).unwrap();

        lua.globals().set("Console", console).unwrap();
    }
}

//fn console_update(mut console: ResMut<Console>, rx: ResMut<ConsoleRx>) {
fn console_update(mut console: ResMut<Console>) {
    while let Some(msg) = CONSOLE_CHANNEL.read() {
        match msg {
            ConsoleMsg::Log(s) => {
                console.entry.push_back(s);
                if console.entry.len() > 1000 {
                    console.entry.pop_front();
                }
            }
            ConsoleMsg::Clear => console.entry.clear(),
        }
    }
}

fn console_show(mut ctx: EguiContexts, console: Res<Console>) {
    egui::Window::new("Console")
        .default_open(true)
        .default_size([250.0, 500.0])
        .default_pos([900.0, 20.0])
        .vscroll(true)
        .scroll(true)
        .resizable(true)
        .show(ctx.ctx_mut(), |ui| {
            egui::Grid::new("console_main_grid")
                .max_col_width(750.0)
                .striped(true)
                .show(ui, |ui| {
                    for entry in console.entry.iter() {
                        ui.label(entry);
                        ui.end_row();
                    }
                });
        });
}
