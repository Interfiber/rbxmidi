use fltk::{
    app,
    button::*,
    frame::Frame,
    group::{Group, Pack, Tabs},
    prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
    window::Window,
    browser::HoldBrowser,
    prelude::BrowserExt,
};

mod midi;
mod agent;
mod state;
mod key;

fn load_list(mut list: HoldBrowser) {
    let devices = midi::finder::find_midi_devices();
    for i in devices {
        println!("adding device: {}", i);
        list.add(&i);
    }
}

fn draw_gallery() {
    let tab = Tabs::new(10, 10, 500 - 20, 450 - 20, "");

    let grp1 = Group::new(10, 35, 500 - 20, 450 - 45, "General\t\t");
    let mut pack = Pack::new(15, 45, 150, 450 - 45, "");
    pack.set_spacing(10);
    let _title_label = Frame::default()
        .with_size(100, 40)
        .with_label("Midi to Roblox Piano");
   
    let mut toggle_button = Button::default()
        .with_label("Toggle RBX Midi")
        .center_of(&pack)
        .with_size(50, 50);
    toggle_button.set_callback(move |_| {
        println!("checking state...");
        let state_type = state::get_state();
        if state_type == state::StateType::Enabled {
            println!("state is: enabled, disabling");
        } else {
            println!("state is: disabled, enabling");
            agent::init::start_agent();
        }
    });
    pack.end();
    grp1.end();

    let grp2 = Group::new(10, 35, 500 - 30, 450 - 25, "Settings\t\t");
    let mut pack = Pack::new(15, 45, 150, 450 - 45, "");
    pack.set_spacing(10);
    println!("scanning for midi devices");
    let _midi_label = Frame::default()
        .with_size(100, 40)
        .with_label("Select MIDI device:");
    let mut list = HoldBrowser::default()
        .with_size(700, 200);
    load_list(list.clone());
    list.add("Reload Devices");
    list.clone().set_callback(move |_| {
        let selected = list.selected_text();
        if selected.is_none(){
            println!("nothing selected, skipping");
        } else {
            let selected_unwrap = selected.unwrap();
            if selected_unwrap == "Reload Devices" {
                println!("reloading device list");
                list.clear();
                load_list(list.clone());
                list.add("Reload Devices");
                println!("reloaded device list");
            } else {
                println!("saving device name to /tmp/rbxmidi.devicename");
                state::set_device(selected_unwrap);
            }
        }
    });
    pack.end();
    grp2.end();
    tab.end();
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(221, 221, 221);

    let mut wind = Window::default()
        .with_size(500, 450)
        .with_label("RBXMidi - Play roblox pianos with a MIDI keyboard")
        .center_screen();

    draw_gallery();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
