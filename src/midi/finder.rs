use midir::{MidiInput, Ignore};

pub fn find_midi_devices() -> Vec<String> {
    let mut midi_in = MidiInput::new("midir test input").expect("Failed to create midi input device");
    midi_in.ignore(Ignore::None);
    let mut ports = vec![]; 
    for (_i, p) in midi_in.ports().iter().enumerate() {
            ports.push(midi_in.port_name(p).expect("Failed to get port"));
    }
    println!("scanned for midi devices");
    return ports;
}
