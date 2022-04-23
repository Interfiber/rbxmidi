pub enum Note {
    C4,
    CSharp4,
    D4,
    D4Sharp,
    E4,
    F4,
    F4Sharp,
    InvalidByte
}

pub fn byte_to_enum(byte: u8) -> Note {
    match byte {
        60 => {
            return Note::C4;
        },
        61 => {
            return Note::CSharp4;
        }
        62 => {
            return Note::D4;
        }
        63 => {
            return Note::D4Sharp;
        }
        64 => {
            return Note::E4;
        }
        65 => {
            return Note::F4;
        }
        66 => {
            return Note::F4Sharp;
        }
        _ => {
            println!("Invalid byte");
            return Note::InvalidByte;
        }
    }
}
