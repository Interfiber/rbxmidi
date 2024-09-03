use egui::{Color32, RichText, Ui};
use log::info;

use crate::midi::device::{Device, DeviceManager};

pub struct UserInterface {
    pub device: Device,
    pub device_manager: DeviceManager,
    pub is_connected: bool,
    pub is_activated: bool,
    pub config: Option<crate::config::RobloxMidiConfig>
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
                self.device_manager.connect_device(&mut self.device, self.config.clone().unwrap());

                self.is_connected = true;
            }
        });

        ui.label("");

        ui.horizontal(|ui| {
            if ui.button("Start/stop").clicked() {
                info!("Toggling RBXMIDI");
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
            config: None
        }
    }
}

impl Default for UserInterface {
    fn default() -> Self {
        Self::new()
    }
}
