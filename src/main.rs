use eframe::*;
use egui::CentralPanel;
use std::process::Command;
use std::fs;
use std::path::Path;
use std::env;


struct HeroPak {
    filepath: String,
    package_name: String,
    status_message: String,
}


impl eframe::App for HeroPak {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
   
                ui.heading("HeroPak - MHUR Asset Packager");

                ui.add_space(20.0); 

                ui.vertical_centered(|ui| {
                    ui.label("Name of Assets Root Folder:");
                    ui.text_edit_singleline(&mut self.filepath);

                    ui.add_space(10.0);

                    ui.label("Package Name:");
                    ui.text_edit_singleline(&mut self.package_name);
                });

                ui.add_space(20.0);

                if ui.button("Package Assets").clicked() {

                    let exe_dir = env::current_exe()
                        .ok()
                        .and_then(|path| path.parent().map(|p| p.to_path_buf()))
                        .unwrap_or_else(|| Path::new(".").to_path_buf());


                    let local_repak_path = exe_dir.join("repak.exe");

  
                    let repak_command = if local_repak_path.exists() {
                        local_repak_path
                    } else {
                        Path::new("repak").to_path_buf() // Use global repak if local doesn't exist
                    };


                    let output = Command::new(repak_command)
                        .arg("pack")
                        .arg(&self.filepath)
                        .output();

                    match output {
                        Ok(output) => {
                            if output.status.success() {
                                let input_folder_name = Path::new(&self.filepath)
                                    .file_name()
                                    .unwrap_or_default()
                                    .to_string_lossy()
                                    .to_string();
                                let original_pak_path = format!("{}.pak", &input_folder_name);

                                let new_pak_path = format!("X{}-WindowsNoEditor_P.pak", &self.package_name);
                                if let Err(e) = fs::rename(&original_pak_path, &new_pak_path) {
                                    self.status_message = format!(
                                        "Assets packaged, but failed to rename file: {}", e
                                    );
                                    return;
                                }
                                
                                self.status_message = format!(
                                    "Assets Packaged Successfully! Output:\n{}",
                                    String::from_utf8_lossy(&output.stdout)
                                );
                            } else {
                                self.status_message = format!(
                                    "Failed to Package Assets! Error:\n{}",
                                    String::from_utf8_lossy(&output.stderr)
                                );
                            }
                        }
                        Err(e) => {
                            self.status_message = format!("Failed to Package Assets! Error: {}", e);
                        }
                    }
                }

                ui.add_space(20.0);

                if !self.status_message.is_empty() {
                    ui.label(&self.status_message);
                }
            });
        });
    }
}

fn main() -> eframe::Result<(), eframe::Error> {
    
    run_native(
        "HeroPak", 
        NativeOptions::default(), 
        Box::new(|_cc| {
            Ok(Box::new(HeroPak {
                filepath: String::new(),
                package_name: String::new(),
                status_message: String::new()
            }))
        })
    )
}