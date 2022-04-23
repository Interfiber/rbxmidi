pub enum Note {
    G1,
    G3,
    A0,
    InvalidByte
}

pub fn byte_to_enum(byte: u8) -> Note {
    match byte {
        31 => {
            return Note::G1
        },
        55 => {
            return Note::G3
        },
        21 => {
            return Note::A0
        },
        _ => {
            println!("Invalid byte");
            return Note::InvalidByte;
        }
    }
}
