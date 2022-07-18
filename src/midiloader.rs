use std::fs;

use hashbrown::HashMap;
use midly::{Smf, Format, TrackEvent, TrackEventKind, MidiMessage, MetaMessage};

pub fn load_midi_file(path: &str){
    println!("loading midi file...");
    let data = fs::read(path).expect("Failed to read from midi file");
    println!("parsing midi file...");
    let mut smf = Smf::parse(&data).expect("Failed to parse midi file");
    smf.header.format = Format::Sequential;
    println!("analyzing midi file...");
    let mut note_map = HashMap::new();

    for track in smf.tracks {
	for event in track {
	    match event.kind {
		TrackEventKind::Midi { channel, message } => match message {
		    MidiMessage::NoteOn { key, vel } => {
			let note = crate::midi::convert::byte_to_enum(key.as_int());
			println!("Note on: {}", key);
			note_map.insert(key, true);
			println!("{:#?}", note_map);
//			crate::key::note_press(note);
		    },
		    MidiMessage::NoteOff { key, vel } => {
			println!("Note off: {}", key);
			note_map.remove(&key);
			note_map.insert(key, false);
			println!("{:#?}", note_map);
		    },
		    _ => println!("Unhandled type, type is: {:#?}", message)
		},
		TrackEventKind::Meta(message) => match message {
		    MetaMessage::TimeSignature(num, denom, cpt, note_per_quarter) => {
		    },
		    _ => println!("Unhandled metamessage type")
		}
		_ => println!("Unhandled type for event.kind, kind is: {:#?}", event.kind)
	    }
	}
    }
}
