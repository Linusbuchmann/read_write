#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use std::fs::File;
use std::io::Write;
use std::io::Read;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).


    let options = eframe::NativeOptions {
        max_window_size: Some(egui::vec2(345.0, 150.0)), 
        resizable: true,
        initial_window_size: Some(egui::vec2(345.0, 150.0)),
        ..Default::default()
    };

    // Our application state:

    let mut input_data = "write something".to_owned();

    let mut data_file = File::open("data.txt").unwrap();

    let mut file_content = String::new();

    eframe::run_simple_native("", options, move |ctx, _frame| {
        data_file.read_to_string(&mut file_content).unwrap();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("write and read to file");
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.text_edit_singleline(&mut input_data);
                
                if ui.button("write").clicked() {
                    let mut data_file = File::create("data.txt").expect("creation failed");
                    data_file.write(input_data.as_bytes()).expect("write failed");
                }
                
            });
            

            ui.label(format!("text: {file_content}"));

        });
    })
}