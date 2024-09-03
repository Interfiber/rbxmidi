use log::info;
use midir::MidiInput;
use std::sync::mpsc::Sender;

use super::background::WorkerTaskPacket;

extern crate midi_control;
extern crate midir;

pub struct Device {
    pub name: String,
    pub is_connected: bool,
}

pub struct DeviceManager {
    pub device_list: Vec<Device>,
    pub input: MidiInput,
}

impl DeviceManager {
    pub fn default() -> DeviceManager {
        DeviceManager {
            device_list: vec![],
            input: midir::MidiInput::new("RBXMidi").unwrap(),
        }
    }

    pub fn connect_device(&self, device: &mut Device, sender: Option<Sender<WorkerTaskPacket>>) {
        device.is_connected = true;

        let packet = WorkerTaskPacket {
            task_type: crate::midi::background::WorkerTaskType::DeviceName,
            data: device.name.clone()
        };

        sender.unwrap().send(packet).expect("Failed to send device name to worker thread");

        info!("Prompted connection to device '{}'", device.name);
    }
}

impl Device {
    pub fn default() -> Device {
        Device {
            name: String::from("No Device"),
            is_connected: false,
        }
    }

    pub fn get_devices() -> DeviceManager {
        let midi_input = midir::MidiInput::new("RBXMidi").unwrap();

        info!("Scanning midi devices...");

        let mut devices: Vec<Device> = vec![];

        for (_i, port) in midi_input.ports().iter().enumerate() {
            if let Ok(port_name) = midi_input.port_name(&port) {
                info!("Found device with name '{}'", port_name);

                let device: Device = Device {
                    name: port_name,
                    is_connected: false,
                };

                devices.push(device);
            }
        }

        return DeviceManager {
            device_list: devices,
            input: midi_input,
        };
    }
}
