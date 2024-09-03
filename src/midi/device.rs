use log::{debug, error, info, trace};
use midi_control::MidiMessage;
use midir::{MidiInput, MidiInputPort};
use std::{sync::mpsc::channel, time::Duration};

use crate::{config::RobloxMidiConfig, midi::data::Note};

extern crate midi_control;
extern crate midir;

pub struct Device {
    pub name: String,
    pub sender: Option<std::sync::mpsc::Sender<MidiMessage>>,
    pub receiver: Option<std::sync::mpsc::Receiver<MidiMessage>>,
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

    pub fn connect_device(&self, device: &mut Device, config: RobloxMidiConfig) {
        let mut port: Option<MidiInputPort> = None;

        // We need to create a new input device....
        let mut input = MidiInput::new("RobloxMidi").unwrap();
        input.ignore(midir::Ignore::None);

        for t_port in input.ports() {
            if device.name == input.port_name(&t_port).expect("No device name!") {
                port = Some(t_port);

                break;
            }
        }

        if port.is_none() {
            error!("Failed to find device!");

            return;
        }

        let (sender, receiver) = channel::<MidiMessage>();

        device.is_connected = true;

        device.sender = Some(sender.clone());
        device.receiver = Some(receiver);

        let _conn_in = input
            .connect(
                &port.clone().expect("No port on device"),
                "midir-read-input",
                move |_timestamp, data, sender| {
                    let msg = MidiMessage::from(data);

                    match msg {
                        MidiMessage::NoteOn(_channel, keyEvent) => {
                            let note = Note::to_enum(keyEvent.key);
                            let key = match config.keys.get(&note) {
                                Some(k) => k,
                                None => {
                                    error!("Invalid note: {:#?}", note);

                                    return;
                                }
                            };

                            debug!("{}", key);
                        }

                        _ => trace!("Ignoring midi message of type: {:#?}", msg),
                    };

                    
                },
                (),
            )
            .expect("Failed to connect to device!");

        info!("Connected to device '{}'", device.name);

        loop {
            std::thread::sleep(Duration::from_millis(100)); // FIXME: Somehow push device code to another thread
        }
    }
}

impl Device {
    pub fn default() -> Device {
        Device {
            name: String::from("No Device"),
            sender: None,
            receiver: None,
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
                    sender: None,
                    receiver: None,
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
