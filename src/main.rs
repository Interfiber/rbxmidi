#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

pub mod ui;
pub mod midi;
pub mod config;

use eframe::egui;
use log::info;
use ui::main_ui::UserInterface;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    info!("Welcome to RBXMIDI!");

    let mut user_interface: ui::main_ui::UserInterface = UserInterface::new();

    user_interface.device_manager = midi::device::Device::get_devices();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    eframe::run_simple_native("Roblox MIDI Piano", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            user_interface.render(ui);
        });
    })
}
