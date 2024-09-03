use std::sync::mpsc::Sender;

use egui::{Color32, RichText, Ui};
use log::info;

use crate::midi::{background::WorkerTaskPacket, device::{Device, DeviceManager}};

pub struct UserInterface {
    pub device: Device,
    pub device_manager: DeviceManager,
    pub is_connected: bool,
    pub is_activated: bool,
    pub device_thread_sender: Option<Sender<WorkerTaskPacket>>
}

impl UserInterface {
    pub fn render(&mut self, ui: &mut Ui) {
        ui.heading("Roblox Midi v2.0");
        // ui.label("Connected: No\tActivated: No");

        ui.horizontal(|ui| {
            ui.label("Connected ");

            if self.is_connected {
                ui.label(RichText::new("Yes").color(Color32::GREEN));
            } else {
                ui.label(RichText::new("No").color(Color32::RED));
            }

            ui.label("\tActivated ");

            if self.is_activated {
                ui.label(RichText::new("Yes").color(Color32::GREEN));
            } else {
                ui.label(RichText::new("No").color(Color32::RED));
            }
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("MIDI device");

            egui::ComboBox::new("midiDevice", "")
                .selected_text(self.device.name.clone())
                .show_ui(ui, |ui| {
                    for device in &self.device_manager.device_list {
                        ui.selectable_value(
                            &mut self.device.name,
                            device.name.clone(),
                            device.name.clone(),
                        );
                    }
                });

            if self.is_connected {
                ui.disable();
            }

            if ui.button("Connect").clicked() {
                self.device_manager.connect_device(&mut self.device, self.device_thread_sender.clone());

                self.is_connected = true;
                self.is_activated = true;
            }
        });

        ui.label("");

        ui.horizontal(|ui| {
            if ui.button("Start/stop").clicked() {
                info!("Toggling RBXMIDI");

                self.is_activated = !self.is_activated;

                let packet = WorkerTaskPacket {
                    task_type: crate::midi::background::WorkerTaskType::Pause,
                    data: (!self.is_activated).to_string()
                };

                self.device_thread_sender.clone().unwrap().send(packet).expect("Failed to send pause packet");
            }

            if ui.button("Quit RBXMidi").clicked() {
                std::process::exit(0);
            }
        });
    }

    pub fn new() -> UserInterface {
        UserInterface {
            device: Device::default(),
            device_manager: DeviceManager::default(),
            is_connected: false,
            is_activated: false,
            device_thread_sender: None
        }
    }
}

impl Default for UserInterface {
    fn default() -> Self {
        Self::new()
    }
}
