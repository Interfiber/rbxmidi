use enigo::{Enigo, Key, KeyboardControllable};
use std::time::Duration;
use crate::midi::convert::Note;

pub fn press_key(key: String){
    std::thread::sleep(Duration::from_secs(2));
    let mut enigo = Enigo::new();
    let key_char: Vec<char> = key.chars().collect();
    enigo.key_down(Key::Layout(key_char[0]));
    enigo.key_up(Key::Layout(key_char[0]));
}

pub fn note_press(note: Note) {
    // match the note enum, then press the key
    match note {
        Note::C4 => {
            press_key("t".to_string());
        }
        Note::CSharp4 => {
            press_key("T".to_string());
        },
        Note::D4 => {
            press_key("y".to_string());
        }
        Note::D4Sharp => {
            press_key("Y".to_string());
        }
        Note::E4 => {
            press_key("u".to_string());
        }
        Note::F4 => {
            press_key("i".to_string());
        }
        Note::F4Sharp => {
            press_key("I".to_string());
        }
        _ => println!("invalid type")
    }
}
