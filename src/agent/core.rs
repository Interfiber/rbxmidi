use std::io::{BufRead, BufReader};
use std::os::unix::net::{UnixStream,UnixListener};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::thread::Builder;

static MIDI_CONTENT: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![]));

fn handle_client(stream: UnixStream) {
    let stream = BufReader::new(stream);
    for line in stream.lines() {
        let data = line.unwrap();
        if data == "midi.activate_worker" {
            println!("Spawning background worker to listen to MIDI");
            Builder::new()
                .name("rbxmidi_agent_midi_worker".to_string())
                .spawn(|| crate::agent::midi::listen_midi())
                .expect("Failed to spawn MIDI worker");
        } else if data == "midi.cancel_worker" {
            println!("Creating midi worker shutdown file");
            std::fs::write("/tmp/rbxmidi.midi_worker_stop", "").expect("Failed to create stop file");
        } else if data == "midi.clear_midi_file" {

	} else if data.contains("midi.load_midi_file") {
	    let args: Vec<&str> = data.split(":").collect();
	    if args.len() != 2 {
		println!("Agent error: Not enough arguments for load midi file: expected 2!");
	    } else {
		println!("Loading midi file: {}", args[1]);
		crate::midiloader::load_midi_file(args[1]);
	    }
	} else if data == "midi.play_midi_file" {
	}
    }
}

pub fn agent_main() {
    println!("RBX midi agent started(agent thread)");
    // listen on the socket
    let listener = UnixListener::bind("/tmp/rbxmidi_agent.sock").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Got connection, spawning stream worker");
                Builder::new()
                    .name("rbxmidi_agent_stream_worker".to_string())
                    .spawn(|| handle_client(stream))
                    .expect("Failed to spawn agent stream worker thread");
            },
            Err(err) => {
                println!("Error: {}", err);
            }
        }
    }
    println!("RBX midi agent socket located at: /tmp/rbxmidi_agent.sock");
}
