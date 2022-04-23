use fltk::{
    app,
    button::*,
    frame::Frame,
    group::{Group, Pack, Tabs},
    prelude::{GroupExt, WidgetBase, WidgetExt, WindowExt},
    window::Window,
};

mod midi;

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
    let devices = midi::finder::find_midi_devices();
    for i in devices {
        println!("adding device: {}", i);
        let _but2 = RoundButton::default().with_size(0, 30).with_label(&i);
    }
    pack.end();
    grp2.end();
    tab.end();
}

fn main() {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(221, 221, 221);

    let mut wind = Window::default()
        .with_size(500, 450)
        .with_label("Tabs")
        .center_screen();

    draw_gallery();

    wind.make_resizable(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
