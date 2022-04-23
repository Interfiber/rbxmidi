static STATE: state::Storage<u32> = state::Storage::new();
// States:
// 0 = Disabled
// 1 = Enabled

#[derive(PartialEq)]
pub enum StateType {
    Enabled,
    Disabled
}

pub fn get_state() -> StateType {
    let state = STATE.try_get();
    if state.is_none() {
        println!("state is empty, setting to 0");
        STATE.set(0);
        return StateType::Disabled;
    } else {
        let state_unwrapped = *STATE.get();
        if state_unwrapped == 0 {
            return StateType::Disabled;
        } else {
            return StateType::Enabled;
        }
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
