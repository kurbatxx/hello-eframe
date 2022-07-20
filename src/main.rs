#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{thread, time::Duration};

use eframe::egui;
use poll_promise::Promise;

fn main() {
    let options = eframe::NativeOptions {
        min_window_size: Some(egui::vec2(320.0, 100.0)),
        initial_window_size: Some(egui::vec2(500.0, 400.0)),
        vsync: false,
        ..Default::default()
    };
    eframe::run_native("App", options, Box::new(|_cc| Box::new(MyApp::default())));
}

struct MyApp {
    name: String,
    age: u32,
    promise: Option<Promise<i32>>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            age: 42,
            promise: None,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Start app");
            });

            ui.with_layout(
                egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
                |ui| {
                    ui.text_edit_singleline(&mut self.name);
                },
            );
            if ui.button("Click each year").clicked() {
                self.age += 1;
                let ctx = ctx.clone();

                //let promise = poll_promise::Promise::spawn_thread("slow_operation", move || slow());
                let (sender, promise) = Promise::new();

                std::thread::spawn(move || {
                    println!("run slow operation");
                    thread::sleep(Duration::from_secs(4));
                    sender.send(42);
                    ctx.request_repaint();
                });
                self.promise = Some(promise)
            }

            if let Some(promise) = &self.promise {
                match promise.ready() {
                    None => {
                        ui.spinner();
                    }
                    Some(_) => {
                        ui.label("result");
                    }
                }
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
