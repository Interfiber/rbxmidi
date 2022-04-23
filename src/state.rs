pub fn midi_worker_is_enabled() -> bool { 
    if std::path::Path::new("/tmp/rbxmidi_midi_worker").exists() {
        return true;
    } else {
        return false;
    }
}
pub fn set_device(device_name: String) {
    match std::fs::write("/tmp/rbxmidi.devicename", device_name){
        Ok(_) => println!("saved device"),
        Err(err) => {
            println!("failed to save device");
            println!("Error log: {}", err);
        }
    }
}

pub fn get_device() -> String {
    let data = std::fs::read_to_string("/tmp/rbxmidi.devicename").expect("Failed to read cached devicename");
    return data.to_string();
}
