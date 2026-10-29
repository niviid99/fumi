use std::{env, fs, sync::Arc};

use crate::fmgr;
use std::path;

#[derive(PartialEq, Clone)]
enum WorkMode {
    Interact,
    Selection,
}

impl WorkMode {
    fn name(&self) -> &'static str {
        match self {
            WorkMode::Interact => "Interact",
            WorkMode::Selection => "Select",
        }
    }
}

pub struct Main {
    show_hidden_files: bool,
    files_list: Vec<fmgr::File>,
    read_path: String,
    work_mode: WorkMode,
    selected_file: Option<fmgr::File>,
    new_path: String,
    new_file_name: String,
    ff: bool,
}

impl Default for Main {
    fn default() -> Self {
        let current_path = env::var("PWD").unwrap_or("/".to_string());
        Self {
            show_hidden_files: false,
            files_list: Vec::new(),
            read_path: current_path,
            work_mode: WorkMode::Interact,
            selected_file: None,
            new_path: String::new(),
            new_file_name: String::new(),
            ff: true,
        }
    }
}

impl eframe::App for Main {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.ff {
            self.setup_font(ui);
            self.update_list();
            self.ff = false;
        }

        egui::Panel::top("top_panel").show(ui, |ui| {
            ui.heading("FUMI v0.1.0");

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.read_path);

                ui.checkbox(&mut self.show_hidden_files, "hidden files");

                egui::ComboBox::from_label("Mode")
                    .selected_text(self.work_mode.name())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.work_mode,
                            WorkMode::Interact,
                            WorkMode::Interact.name(),
                        );
                        ui.selectable_value(
                            &mut self.work_mode,
                            WorkMode::Selection,
                            WorkMode::Selection.name(),
                        );
                    });
            });
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_file_name);
                if ui.button("Create file").clicked() {
                    let err = fs::write(format!("{}/{}", self.read_path, self.new_file_name), "");
                    if err.is_err() {
                        println!("Failed to create file");
                    }
                    self.update_list();
                }
                if ui.button("Create dir").clicked() {
                    let err = fs::create_dir(format!("{}/{}", self.read_path, self.new_file_name));
                    if err.is_err() {
                        println!("Failed to create dir");
                    }
                    self.update_list();
                }
            });
            if ui.button("update").clicked() {
                self.update_list();
            }
        });
        egui::Panel::bottom("bottom_panel").show(ui, |ui| {
            self.selection_worker(ui);
        });
        egui::CentralPanel::default().show(ui, |ui| {
            let mut np: Option<String> = None;
            let mut slp: Option<fmgr::File> = None;

            if ui.button("..").clicked() {
                let pth = path::Path::new(&self.read_path).parent();
                if let Some(p) = pth {
                    np = Some(p.to_string_lossy().to_string())
                }
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                for file in &self.files_list {
                    if ui.button(file.name.to_string()).clicked() {
                        if self.work_mode == WorkMode::Interact
                            && file.filetype == fmgr::FileType::Dir
                        {
                            np = Some(file.path.to_string());
                        }
                        if self.work_mode == WorkMode::Selection {
                            slp = Some((*file).clone());
                        }
                    }
                }
            });

            if let Some(selection) = slp {
                self.new_path = selection.path.clone();
                self.selected_file = Some(selection.clone());
            }
            if let Some(path) = np {
                self.read_path = path;
                self.update_list();
            }
        });
    }
}

impl Main {
    fn selection_worker(&mut self, ui: &mut egui::Ui) {
        if self.work_mode == WorkMode::Selection {
            if let Some(sf) = &self.selected_file {
                let current_fname = sf.name.clone();
                let current_path = sf.path.clone();
                let current_type = sf.filetype.clone();
                ui.heading(&current_fname);
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.new_path);
                    if ui.button("Move").clicked() {
                        let res = fs::rename(&current_path, &self.new_path);
                        if res.is_err() {
                            println!("Failed to rename {} to {}", current_path, self.new_path);
                        }
                        self.selected_file = None;
                        self.update_list();
                    }
                    if ui.button("Copy").clicked() {
                        let res = fs::copy(&current_path, &self.new_path);
                        if res.is_err() {
                            println!("Failed to copy {} to {}", current_path, self.new_path);
                        }
                        self.selected_file = None;
                        self.update_list();
                    }
                });
                ui.horizontal(|ui| {
                    if ui.button("Remove").clicked() {
                        if current_type == fmgr::FileType::Dir {
                            let err = fs::remove_dir_all(&current_path);
                            if err.is_err() {
                                println!("Failed to remove dir {}", current_path);
                            }
                            self.selected_file = None;
                        } else {
                            let err = fs::remove_file(&current_path);
                            if err.is_err() {
                                println!("Failed to remove file {}", current_path);
                            }
                            self.selected_file = None;
                        }
                        self.update_list();
                    }
                });
            }
        }
    }
    fn update_list(&mut self) {
        let res = fmgr::ls_to_arr(
            &mut self.files_list,
            &self.read_path,
            fmgr::BasicConf {
                hidden_files: self.show_hidden_files,
                ..fmgr::BasicConf::default()
            },
        );
        if res.is_err() {
            self.files_list.push(fmgr::File {
                name: "Failed to read path".to_string(),
                ..fmgr::File::default()
            });
        }
    }
    fn setup_font(&self, ui: &egui::Ui) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "my_nerd_font".to_owned(),
            Arc::new(egui::FontData::from_static(include_bytes!(
                "../AgaveNerdFont-Regular.ttf"
            ))),
        );

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "my_nerd_font".to_owned());

        fonts
            .families
            .get_mut(&egui::FontFamily::Monospace)
            .unwrap()
            .insert(0, "my_nerd_font".to_owned());

        ui.set_fonts(fonts);
    }
}
