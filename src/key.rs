use crate::midi::convert::Note;

#[cfg(target_os = "macos")]
fn macos_press_key(key: String){
    let key_char: Vec<char> = key.chars().collect();
    mkeypress::send_key_wrap(key_char[0]);
}

#[cfg(target_os = "linux")]
fn linux_press_key(key: String){
    use enigo::Enigo;
    use enigo::Key;
    use enigo::KeyboardControllable;
    let mut enigo = Enigo::new();
    let key_char: Vec<char> = key.chars().collect();
    enigo.key_down(Key::Layout(key_char[0]));
    enigo.key_up(Key::Layout(key_char[0]));
}

pub fn press_key(key: String){
    #[cfg(target_os = "linux")]
    linux_press_key(key);

    #[cfg(target_os = "macos")]
    macos_press_key(key);
}

pub fn note_press(note: Note) {
    // match the note enum, then press the key
    match note {
        Note::C2 => {
            press_key("1".to_string());
        }
        Note::C2Sharp => {
            press_key("!".to_string());
        },
        Note::D2 => {
            press_key("2".to_string());
        }
        Note::D2Sharp => {
            press_key("@".to_string());
        }
        Note::E2 => {
            press_key("3".to_string());
        }
        Note::F2 => {
            press_key("4".to_string());
        }
        Note::F2Sharp => {
            press_key("$".to_string());
        }
        Note::G2 => {
            press_key("5".to_string());
        }
        Note::G2Sharp => {
            press_key("%".to_string());
        }
        Note::A2 => {
            press_key("6".to_string());
        }
        Note::A2Sharp => {
            press_key("^".to_string());
        }
        Note::B2 => {
            press_key("7".to_string());
        }
        Note::C3 => {
            press_key("8".to_string());
        }
        Note::C3Sharp => {
            press_key("*".to_string());
        },
        Note::D3 => {
            press_key("9".to_string());
        }
        Note::D3Sharp => {
            press_key("(".to_string());
        }
        Note::E3 => {
            press_key("0".to_string());
        }
        Note::F3 => {
            press_key("q".to_string());
        }
        Note::F3Sharp => {
            press_key("Q".to_string());
        }
        Note::G3 => {
            press_key("w".to_string());
        }
        Note::G3Sharp => {
            press_key("W".to_string());
        }
        Note::A3 => {
            press_key("e".to_string());
        }
        Note::A3Sharp => {
            press_key("E".to_string());
        }
        Note::B3 => {
            press_key("r".to_string());
        }
        Note::C4 => {
            press_key("t".to_string());
        }
        Note::C4Sharp => {
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
        Note::G4 => {
            press_key("o".to_string());
        }
        Note::G4Sharp => {
            press_key("O".to_string());
        }
        Note::A4 => {
            press_key("p".to_string());
        }
        Note::A4Sharp => {
            press_key("P".to_string());
        }
        Note::B4 => {
            press_key("a".to_string());
        }
        Note::C5 => {
            press_key("s".to_string());
        }
        Note::C5Sharp => {
            press_key("S".to_string());
        },
        Note::D5 => {
            press_key("d".to_string());
        }
        Note::D5Sharp => {
            press_key("D".to_string());
        }
        Note::E5 => {
            press_key("f".to_string());
        }
        Note::F5 => {
            press_key("g".to_string());
        }
        Note::F5Sharp => {
            press_key("G".to_string());
        }
        Note::G5 => {
            press_key("h".to_string());
        }
        Note::G5Sharp => {
            press_key("H".to_string());
        }
        Note::A5 => {
            press_key("j".to_string());
        }
        Note::A5Sharp => {
            press_key("J".to_string());
        }
        Note::B5 => {
            press_key("k".to_string());
        }
        Note::C6 => {
            press_key("l".to_string());
        }
        Note::C6Sharp => {
            press_key("L".to_string());
        },
        Note::D6 => {
            press_key("z".to_string());
        }
        Note::D6Sharp => {
            press_key("Z".to_string());
        }
        Note::E6 => {
            press_key("x".to_string());
        }
        Note::F6 => {
            press_key("c".to_string());
        }
        Note::F6Sharp => {
            press_key("C".to_string());
        }
        Note::G6 => {
            press_key("v".to_string());
        }
        Note::G6Sharp => {
            press_key("V".to_string());
        }
        Note::A6 => {
            press_key("b".to_string());
        }
        Note::A6Sharp => {
            press_key("B".to_string());
        }
        Note::B6 => {
            press_key("n".to_string());
        }
        Note::C7 => {
            press_key("m".to_string());
        }
        _ => println!("invalid type")
    }
}
