pub enum Note {
    F1,
    G1,
}

pub fn byte_to_enum(byte: u8) -> Note {
    match byte {
        31 => {
            return Note::F1
        },
        _ => {
            println!("Invalid byte");
            println!("Returning f1 for default");
            return Note::F1;
        }
    }
}
