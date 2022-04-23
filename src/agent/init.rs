use std::thread::Builder;

pub fn start_agent(){
    println!("RBXMidi Agent version {}", env!("CARGO_PKG_VERSION"));
    println!("Starting agent background thread...");
    Builder::new()
        .name("rbxmidi_agent".to_string())
        .spawn(move || crate::agent::core::agent_main())
        .expect("Failed to spawn agent thread");
}
