use fltk::{
    app,
    button::*,
    frame::Frame,
    group::{Group, Pack, Tabs},
    prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
    window::Window,
    browser::HoldBrowser,
    prelude::{BrowserExt, InputExt}, input::FileInput,
};

mod midi;
mod agent;
mod state;
mod midiloader;
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
        .with_label("MIDI to Roblox Piano");
   
    let mut toggle_button = Button::default()
        .with_label("Enable RBX MIDI")
        .center_of(&pack)
        .with_size(50, 50);
    let mut play_midi_button = Button::default()
        .with_label("Toggle MIDI file player")
        .center_of(&pack)
        .with_size(70,70);
    let mut midi_file_path = FileInput::default()
        .center_of(&pack)
        .with_size(100, 100);

    midi_file_path.append("/tmp/examplefile.mid").expect("Failed to append data to widget");
    toggle_button.clone().set_callback(move |_| {
        println!("checking state...");
        let enabled = state::midi_worker_is_enabled();
        if enabled {
            println!("state is: enabled, disabling");
            agent::init::stop_midi_worker();
            toggle_button.set_label("Enable RBX MIDI");
        } else {
            println!("state is: disabled, enabling");
            agent::init::start_midi_worker();
            toggle_button.set_label("Disable RBX MIDI");
        }
    });
    play_midi_button.clone().set_callback(move |_| {
	agent::init::start_midi_file_worker(&midi_file_path.value());
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
                let mut selected_line_num = list.value();
                if selected_line_num != 0 {
                    selected_line_num = selected_line_num - 1;
                }
                state::set_device(selected_line_num.to_string());
            }
        }
    });
    pack.end();
    grp2.end();
    let about_group = Group::new(10, 35, 500 - 20, 450 - 45, "About\t\t");
    let mut pack = Pack::new(15, 45, 150, 450 - 45, "");
    pack.set_spacing(10);

    let _title_frame = Frame::default()
        .with_label("RBX MIDI authors:")
        .center_of_parent()
        .with_size(100, 40);
    
    let _author_one_frame = Frame::default()
        .with_label("Interfiber:\nBase program")
        .with_size(100, 60);

    let _author_two_frame = Frame::default()
        .with_label("antiLimit:\nAdded keys")
        .with_size(100, 60);

    
    pack.end();
    about_group.end();

    tab.end();
}

fn main() {
    agent::init::start_agent();
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(221, 221, 221);

    let mut wind = Window::default()
        .with_size(500, 450)
        .with_label("RBXMIDI - Play roblox pianos with a MIDI keyboard")
        .center_screen();

    draw_gallery();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
