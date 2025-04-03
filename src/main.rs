use eframe::*;
use egui::CentralPanel;
use egui::Layout;
use egui::Direction;
use egui::Vec2;

use std::process::Command;
use std::fs;
use std::path::Path;
use std::env;
use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};


struct HeroPak {
    filepath: String,
    package_name: String,
    status_message: String,
    modsfolder: String,
    show_settings: bool,
    automatically_move_pak: bool,
    config: Config,
}

#[derive(Serialize, Deserialize)]
struct Config {
    modsfolder: String,
    automatically_move_pak: bool,
}

impl Config {
    fn load() -> Self {
        if let Ok(contents) = std::fs::read_to_string("settings.conf") {
            serde_json::from_str(&contents).unwrap_or_else(|_| Config::default())
        } else {
            let config = Config::default();
            config.save();
            config
        }
    }

    fn save(&self) {
        if let Ok(serialized) = serde_json::to_string_pretty(self) {
            let mut file = File::create("settings.conf").unwrap();
            file.write_all(serialized.as_bytes()).unwrap();
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            modsfolder: String::from("C:/Program Files (x86)/Steam/steamapps/common/My Hero Ultra Rumble/HerovsGame/Content/Paks/Mods"),
            automatically_move_pak: false,
        }
    }
}

impl HeroPak {
    
    fn execute_repak(&mut self, command: &str, args: &[&str]) -> Result<String, String> {
        let exe_dir = env::current_exe()
            .ok()
            .and_then(|path| path.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| Path::new(".").to_path_buf());

        let local_repak_path = exe_dir.join("repak.exe");
        let repak_command = if local_repak_path.exists() {
            local_repak_path
        } else {
            Path::new("repak").to_path_buf()
        };

        let mut cmd = Command::new(repak_command);
        cmd.arg(command);
        for arg in args {
            cmd.arg(arg);
        }

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&output.stderr).to_string())
                }
            }
            Err(e) => Err(e.to_string())
        }
    }
}

impl eframe::App for HeroPak {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {

        CentralPanel::default().show(ctx, |ui| {
            if ui.button("⚙").clicked() {
                self.show_settings = !self.show_settings
            }


            egui::Window::new("Settings")
            .open(&mut self.show_settings)
            .show(ctx, |ui| {
                ui.heading("Settings");
                
                ui.add_space(8.0);
                
                ui.label("Path to Mods Folder");
                ui.text_edit_singleline(&mut self.modsfolder);

                ui.add_space(8.0);
                ui.checkbox(&mut self.automatically_move_pak, "Automatically move pak to Mods folder");

                ui.add_space(8.0);
                if ui.button("Save Settings").clicked() {
                    self.config.modsfolder = self.modsfolder.clone();
                    self.config.automatically_move_pak = self.automatically_move_pak;
                    self.config.save();
                }

                ui.add_space(8.0);
                ui.label("version 1.3.0");
            });
        
            
            ui.vertical_centered(|ui| {
   
                ui.heading("HeroPak - MHUR Asset Packager");

                ui.add_space(20.0); 

                ui.vertical_centered(|ui| {
                    ui.label("Name of Assets Root Folder:");
                    ui.text_edit_singleline(&mut self.filepath);

                    ui.add_space(10.0);

                    ui.label("Package Name (OPTIONAL):");
                    ui.text_edit_singleline(&mut self.package_name);

                    /* ui.add_space(10.0);

                    ui.label("Path to Mods Folder (OPTIONAL):");
                    ui.text_edit_singleline(&mut self.modsfolder); */
                });

                ui.add_space(20.0);

                let num_columns = if self.automatically_move_pak { 2 } else { 3 };

                ui.columns(num_columns, |columns| {
                    columns[0].allocate_ui_with_layout(
                        Vec2::ZERO,
                        Layout::centered_and_justified(Direction::RightToLeft),
                        |ui| {
                            if ui.button("Package Assets").clicked() {
                                let filepath_clone = self.filepath.clone();
                                match self.execute_repak("pack", &[&filepath_clone]) {
                                    Ok(output) => {
                                        let input_folder_name = Path::new(&self.filepath)
                                            .file_name()
                                            .unwrap_or_default()
                                            .to_string_lossy()
                                            .to_string();
                                        let original_pak_path = format!("{}.pak", &input_folder_name);
                                        let new_pak_path = format!("X{}-WindowsNoEditor_P.pak", &self.package_name);

                                        if let Err(e) = fs::rename(&original_pak_path, &new_pak_path) {
                                            self.status_message = format!("Assets packaged, but failed to rename file:\n{}", e);
                                        } else {
                                            self.status_message = format!("Packaging successful:\n{}", output);

                                            if self.automatically_move_pak {
                                                let source_path = Path::new(&new_pak_path);
                                                let destination_path = Path::new(&self.modsfolder).join(&new_pak_path);

                                                match fs::copy(source_path, &destination_path) {
                                                    Ok(_) => {
                                                        match fs::remove_file(source_path) {
                                                            Ok(_) => {
                                                                self.status_message = format!("Pak moved to Mods folder:\n{}", destination_path.display());
                                                            }
                                                            Err(e) => {
                                                                self.status_message = format!("Copied pak but failed to remove original:\n{}", e);
                                                            }
                                                        }
                                                    }
                                                    Err(e) => {
                                                        self.status_message = format!("Failed to move pak:\n{}", e);
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => self.status_message = format!("Packaging failed:\n{}", e),
                                }
                            }
                        },
                    );

                    columns[1].allocate_ui_with_layout(
                        Vec2::ZERO,
                        Layout::centered_and_justified(Direction::LeftToRight),
                        |ui| {
                            if ui.button("List Assets").clicked() {
                                let pak_name = format!("X{}-WindowsNoEditor_P.pak", &self.package_name);
                                match self.execute_repak("list", &[&pak_name]) {
                                    Ok(output) => self.status_message = format!("List of Assets\n{}", output),
                                    Err(e) => self.status_message = format!("Failed to list assets:\n{}", e),
                                }
                            }
                        },
                    );

                    if !self.automatically_move_pak {
                        columns[2].allocate_ui_with_layout(
                            Vec2::ZERO,
                            Layout::centered_and_justified(Direction::LeftToRight),
                            |ui|{
                                if ui.button("Move Pak to Mods folder").clicked() {
                                    let pak_name = format!("X{}-WindowsNoEditor_P.pak", &self.package_name);
                                    let source_path = Path::new(&pak_name);
                                    let destination_path = Path::new(&self.modsfolder).join(&pak_name);


                                    
                                    match fs::copy(source_path, &destination_path) {
                                        Ok(_) => {
                                            match fs::remove_file(source_path) {
                                                Ok(_) => {
                                                    self.status_message = format!("Pak moved to Mods folder:\n{}", destination_path.display());
                                                }
                                                Err(e) => {
                                                    self.status_message = format!("Copied pak but failed to remove original:\n{}", e);
                                                }
                                            }
                                            
                                        }
                                        Err(e) => {
                                            self.status_message = format!("Failed to move pak:\n{}", e);
                                        }
                                    }
                                }
                            }
                        );
                    return;
                    }
                });

                ui.add_space(20.0);

                if !self.status_message.is_empty() {
                    ui.label(&self.status_message);
                    }
                });
            });
        }
    }

fn main() -> eframe::Result<(), eframe::Error> {


    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 300.0]),
        ..Default::default()
    };

    run_native(
        "HeroPak", 
        options, 
        Box::new(|_cc| {
            let config = Config::load();
            Ok(Box::new(HeroPak {
                filepath: String::new(),
                package_name: String::new(),
                status_message: String::new(),
                modsfolder: config.modsfolder.clone(),
                show_settings: false,
                automatically_move_pak: config.automatically_move_pak,
                config,
            }))

        })
    )
}