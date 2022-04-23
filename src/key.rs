use enigo::{Enigo, Key, KeyboardControllable};
use crate::midi::convert::Note;

pub fn press_key(key: String){
    let mut enigo = Enigo::new();
    let key_char: Vec<char> = key.chars().collect();
    enigo.key_down(Key::Layout(key_char[0]));
    enigo.key_up(Key::Layout(key_char[0]));
}

pub fn note_press(note: Note) {
    // match the note enum, then press the key
    match note {
        Note::G1 => {
            press_key("7".to_string());
        }
        Note::G3 => {
            press_key("a".to_string());
        }
        Note::A0 => {
            press_key("1".to_string());
        }
        _ => println!("invalid type")
    }
}
