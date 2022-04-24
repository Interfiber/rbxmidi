use midir::{MidiInput, Ignore};
use crate::key::note_press;
use std::time::Duration;
use std::path::Path;

pub fn listen_midi(){
    println!("Starting MIDI listener");
    std::fs::write("/tmp/rbxmidi_midi_worker", "").expect("Failed to create worker file");
    println!("Reading cached devicename...");
    // get cached devicename
    let device = crate::state::get_device();

    // connect to the midi device
    println!("Connecting to MIDI device...");
    let mut midi_in = MidiInput::new("midir reading input").expect("Failed to open input");
    midi_in.ignore(Ignore::None);
    let in_ports = midi_in.ports();
    let in_port = in_ports.get(device.parse::<usize>().unwrap()).expect("Failed to get MIDI port");
    let input_port_name = midi_in.port_name(in_port).expect("Failed to get port name");

    // listen for keypresses
    let _conn_in = midi_in.connect(in_port, "midir-read-input", move |_, message, _| {
        // https://www.recordingblogs.com/wiki/status-byte-of-a-midi-message
        let message_status = message[0];
        // ignore MIDI clock messages
        if message_status != 248 && message.len() > 1 {
            if message_status == 0x90 {
                let message_data = message[1];
                println!("Message Data: {}", message_data);
                // macos does not emit a note off event, instead it emits a note on with a pitch of
                // zero
                if cfg!(target_os = "macos") {
                    // check if we have 3 indexes
                    if message.len() == 3 {
                        let message_pitch = message[2];
                        if message_pitch != 0 {
                            let converted = crate::midi::convert::byte_to_enum(message_data);
                            note_press(converted);
                        }
                    }
                } else {
                    let converted = crate::midi::convert::byte_to_enum(message_data);
                    note_press(converted);
                }
            }
        }
    }, ()).expect("Failed to connect to device");
    println!("Connection open, reading data from {}", input_port_name);
    loop {
        if Path::new("/tmp/rbxmidi.midi_worker_stop").exists() {
            println!("Exiting, got shutdown message");
            std::fs::remove_file("/tmp/rbxmidi.midi_worker_stop").expect("Failed to remove stop file");
            std::fs::remove_file("/tmp/rbxmidi_midi_worker").expect("Failed to remove midi worker file");
            break;
        }
        std::thread::sleep(Duration::from_secs(3));
    }
}
