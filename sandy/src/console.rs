use crate::*;
use lua::{LuaChip, SandyLua};

pub struct ConsolePlugin;

pub static CONSOLE_CHANNEL: channel::LazyChannel<String> = channel::lazy_channel!();

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Console>()
            .init_state::<ConsoleShow>()
            .add_systems(Update, console_update)
            .add_systems(Update, (console_show).run_if(in_state(ConsoleShow::Show)));
    }
}

#[derive(States, Debug, Clone, Default, PartialEq, Eq, Hash)]
pub enum ConsoleShow {
    #[default]
    Show,
    Hidden,
}

#[derive(Debug, Resource, Default, Deref)]
pub struct Console {
    entry: std::collections::VecDeque<String>,
}

pub struct ConsoleChip;
impl LuaChip for ConsoleChip {
    fn build(&self, lua: &mut SandyLua) {
        let console = lua.create_table().unwrap();

        let print = {
            let ch = CONSOLE_CHANNEL.clone();
            lua.create_function(move |_, msg: mlua::String| {
                ch.send(msg.to_string_lossy());
                Ok(())
            })
            .unwrap()
        };
        console.set("print", print).unwrap();

        lua.globals().set("Console", console).unwrap();
    }
}

//fn console_update(mut console: ResMut<Console>, rx: ResMut<ConsoleRx>) {
fn console_update(mut console: ResMut<Console>) {
    while let Some(msg) = CONSOLE_CHANNEL.read() {
        console.entry.push_back(msg);
        if console.entry.len() > 1000 {
            console.entry.pop_front();
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
