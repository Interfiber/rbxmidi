use std::{thread::Builder, os::unix::net::UnixStream, io::Write};

pub fn start_agent(){
    println!("RBXMidi Agent version {}", env!("CARGO_PKG_VERSION"));
    println!("Setting state...");
    println!("Starting agent background thread...");
    Builder::new()
        .name("rbxmidi_agent".to_string())
        .spawn(move || crate::agent::core::agent_main())
        .expect("Failed to spawn agent thread");
}

pub fn start_midi_worker(){
    println!("connecting to socket");
    let mut stream = UnixStream::connect("/tmp/rbxmidi_agent.sock").expect("Failed to connect to agent socket");
    println!("sending activate message");
    stream.write_all(b"midi.activate_worker").expect("Failed to write to socket");
    println!("sent message to worker");
}

pub fn stop_midi_worker(){
    println!("connecting to socket");
    let mut stream = UnixStream::connect("/tmp/rbxmidi_agent.sock").expect("Failed to connect to agent socket");
    println!("sending cancel message");
    stream.write_all(b"midi.cancel_worker").expect("Failed to write to socket");
    println!("sent message to worker");
}
