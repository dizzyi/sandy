use std::{ops::Deref, path::*, str::FromStr};

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

#[derive(Debug, States, Clone, PartialEq, Eq, Hash)]
struct ConfigShow(bool);

impl Default for ConfigShow {
    fn default() -> Self {
        ConfigShow(true)
    }
}

#[derive(Resource)]
struct FilePicker {
    pub open_dir: std::path::PathBuf,
    pub open_file_dialog: Option<FileDialog>,
}

static CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let dir = dirs::config_dir().unwrap().join("sandy");
    if !dir.is_dir() {
        std::fs::create_dir_all(&dir).unwrap();
    }
    dir
});
static CONFIG_OPEN_DIR: LazyLock<PathBuf> = LazyLock::new(|| CONFIG_DIR.join(".open_dir"));

impl Default for FilePicker {
    fn default() -> Self {
        let open_dir = match std::fs::read_to_string(&*CONFIG_OPEN_DIR) {
            Ok(p) => PathBuf::from_str(&p).unwrap(),
            Err(e) => {
                warn!("Failed to read {:?} : {:?}", CONFIG_OPEN_DIR, e);
                let dir = std::env::current_dir().unwrap();
                FilePicker::write_open_dir(&dir);
                dir
            }
        };

        FilePicker {
            open_dir,
            open_file_dialog: None,
        }
    }
}
impl FilePicker {
    fn write_open_dir(dir: &Path) {
        use std::io::Write;
        let buf = dir.to_str().unwrap().to_string();
        let buf = buf.as_bytes();
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&*CONFIG_OPEN_DIR)
            .unwrap();
        println!("writing {:?} : {:?}", CONFIG_OPEN_DIR, dir);
        f.write_all(buf).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app
        .init_state::<ConfigShow>()
        //.add_event::<ConfigEvent>()
        .add_systems(Update, (menu_egui).run_if(in_state(ConfigShow(true))))
        .init_resource::<FilePicker>()
        // -- 
        ;
    }
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

            let open_btn = ui.button("open lua file");
            if open_btn.clicked() {
                let filter = Box::new({
                    let ext = Some(std::ffi::OsStr::new("lua"));
                    move |path: &std::path::Path| -> bool { path.extension() == ext }
                });
                let mut dialog = FileDialog::open_file(Some(file_picker.open_dir.clone()))
                    .show_files_filter(filter);
                dialog.open();
                file_picker.open_file_dialog = Some(dialog);
            }
            let mut open_dir = file_picker.open_dir.clone();
            if let Some(dialog) = &mut file_picker.open_file_dialog {
                if dialog.show(ui.ctx()).selected() {
                    if let Some(file) = dialog.path() {
                        *corpus_path = lua::CorpusPath(Some(file.to_path_buf()));
                    }
                    let curr_dir = dialog.directory();
                    if curr_dir != open_dir {
                        FilePicker::write_open_dir(dialog.directory());
                        open_dir = curr_dir.to_path_buf();
                    }
                }
            }
            file_picker.open_dir = open_dir;
        });
}
