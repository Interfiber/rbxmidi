use std::{sync::mpsc::{channel, Sender}, time::Duration};

use enigo::{Enigo, Key, Keyboard, Settings};
use log::{debug, error, info, trace};
use midi_control::MidiMessage;
use midir::{MidiInput, MidiInputPort};

use crate::{config, midi::data::Note};

const DATA_PATH: &str = "Library/RobloxMidi/Config.ron";

/**
 * Spawn a background worker
 * Returns a mpsc sender in order to inform the background worker of devices to connect to
 * Can only connect to one device per-session
 */
pub fn spawn_background_worker() -> Sender<String> {
    let (tx, rx) = channel();

    std::thread::spawn(move || {
        debug!("Awaiting device name...");
        
        let device_name = rx.recv().expect("Failed to read from channel");

        let config = config::RobloxMidiConfig::load(DATA_PATH);
        let mut port: Option<MidiInputPort> = None;

        // We need to create a new input device....
        let mut input = MidiInput::new("RobloxMidi").unwrap();
        input.ignore(midir::Ignore::None);

        for t_port in input.ports() {
            if device_name == input.port_name(&t_port).expect("No device name!") {
                port = Some(t_port);

                break;
            }
        }

        if port.is_none() {
            error!("Failed to find device!");

            return;
        }

        let _conn_in = input
            .connect(
                &port.clone().expect("No port on device"),
                "midir-read-input",
                move |_timestamp, data, _| {
                    let msg = MidiMessage::from(data);

                    match msg {
                        MidiMessage::NoteOn(_channel, key_event) => {
                            let note = Note::to_enum(key_event.key);
                            let key = match config.keys.get(&note) {
                                Some(k) => k,
                                None => {
                                    error!("Invalid note: {:#?}", note);

                                    return;
                                }
                            };

                            info!("Pressing key: {}", key);

                            let mut enigo = Enigo::new(&Settings::default()).unwrap();

                            enigo.key(
                                    Key::Unicode(key.as_bytes()[0] as char),
                                    enigo::Direction::Press,
                                )
                                .expect("Failed to press key!");
                        }

                        _ => trace!("Ignoring midi message of type: {:#?}", msg),
                    };
                },
                (),
            )
            .expect("Failed to connect to device!");

        info!("Connected to device '{}'", device_name);

        loop {
            std::thread::sleep(Duration::from_millis(100)); // FIXME: Somehow push device code to another thread
        }
    });

    return tx;
}
