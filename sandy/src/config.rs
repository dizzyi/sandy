use std::path::*;

use lua::CorpusPath;

use crate::*;

//pub fn config_dirs() -> Vec<PathBuf> {
//    vec![
//        dirs::config_dir(),
//        dirs::config_local_dir(),
//        dirs::data_dir(),
//        dirs::data_local_dir(),
//        dirs::home_dir(),
//        dirs::executable_dir(),
//    ]
//    .into_iter()
//    .flatten()
//    .collect()
//}

#[derive(Debug, States, Clone, PartialEq, Eq, Hash, Default)]
enum ConfigShow {
    #[default]
    Show,
    Hidden,
}

#[derive(Resource, Default)]
struct FilePicker {
    pub opened_file: Option<std::path::PathBuf>,
    pub open_file_dialog: Option<FileDialog>,
}

#[derive(Debug, Clone)]
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SandyConfig>()
        .init_state::<ConfigShow>()
        //.add_event::<ConfigEvent>()
        .add_systems(Update, (menu_egui).run_if(in_state(ConfigShow::Show)))
        .init_resource::<FilePicker>()
        // -- 
        ;
    }
}

//#[derive(Debug, Clone, Event)]
//pub enum ConfigEvent {
//    SelectedFile(std::path::PathBuf),
//}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, Resource)]
pub struct SandyConfig {
    projects: Vec<PathBuf>,
}

fn menu_egui(
    mut ctx: EguiContexts,
    mut file_picker: ResMut<FilePicker>,
    mut corpus_path: ResMut<CorpusPath>,
) {
    egui::Window::new("Hello Choom!")
        .default_open(true)
        .default_size([250.0, 200.0])
        .default_pos([20.0, 20.0])
        .vscroll(true)
        .resizable(true)
        .show(ctx.ctx_mut(), |ui| {
            ui.label("Nova!");

            //let directories = config::config_dirs();

            //for d in directories {
            //    ui.label(format!("{:?}", d));
            //}

            let open_btn = ui.button("open");
            if open_btn.clicked() {
                let filter = Box::new({
                    let ext = Some(std::ffi::OsStr::new("lua"));
                    move |path: &std::path::Path| -> bool { path.extension() == ext }
                });
                let mut dialog =
                    FileDialog::open_file(std::env::current_dir().ok()).show_files_filter(filter);
                dialog.open();
                file_picker.open_file_dialog = Some(dialog);
            }

            if let Some(dialog) = &mut file_picker.open_file_dialog {
                if dialog.show(ui.ctx()).selected() {
                    if let Some(file) = dialog.path() {
                        *corpus_path = lua::CorpusPath(file.to_path_buf());
                    }
                }
            }
        });
}
