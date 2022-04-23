use enigo::{Enigo, Key, KeyboardControllable};

pub fn press_key(key: String){
    let mut enigo = Enigo::new();
    let key_char: Vec<char> = key.chars().collect();
    enigo.key_down(Key::Layout(key_char[0]));
    enigo.key_up(Key::Layout(key_char[0]));
}
