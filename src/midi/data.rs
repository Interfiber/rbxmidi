use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq, Clone)]
pub enum Note {
    C2,
    C2Sharp,
    D2,
    D2Sharp,
    E2,
    F2,
    F2Sharp,
    G2,
    G2Sharp,
    A2,
    A2Sharp,
    B2,
    C3,
    C3Sharp,
    D3,
    D3Sharp,
    E3,
    F3,
    F3Sharp,
    G3,
    G3Sharp,
    A3,
    A3Sharp,
    B3,
    C4,
    C4Sharp,
    D4,
    D4Sharp,
    E4,
    F4,
    F4Sharp,
    G4,
    G4Sharp,
    A4,
    A4Sharp,
    B4,
    C5,
    C5Sharp,
    D5,
    D5Sharp,
    E5,
    F5,
    F5Sharp,
    G5,
    G5Sharp,
    A5,
    A5Sharp,
    B5,
    C6,
    C6Sharp,
    D6,
    D6Sharp,
    E6,
    F6,
    F6Sharp,
    G6,
    G6Sharp,
    A6,
    A6Sharp,
    B6,
    C7,
    InvalidByte,
}

impl Note {
    pub fn to_enum(byte: u8) -> Note {
        match byte {
            36 => {
                return Note::C2;
            }
            37 => {
                return Note::C2Sharp;
            }
            38 => {
                return Note::D2;
            }
            39 => {
                return Note::D2Sharp;
            }
            40 => {
                return Note::E2;
            }
            41 => {
                return Note::F2;
            }
            42 => {
                return Note::F2Sharp;
            }
            43 => {
                return Note::G2;
            }
            44 => {
                return Note::G2Sharp;
            }
            45 => {
                return Note::A2;
            }
            46 => {
                return Note::A2Sharp;
            }
            47 => {
                return Note::B2;
            }
            48 => {
                return Note::C3;
            }
            49 => {
                return Note::C3Sharp;
            }
            50 => {
                return Note::D3;
            }
            51 => {
                return Note::D3Sharp;
            }
            52 => {
                return Note::E3;
            }
            53 => {
                return Note::F3;
            }
            54 => {
                return Note::F3Sharp;
            }
            55 => {
                return Note::G3;
            }
            56 => {
                return Note::G3Sharp;
            }
            57 => {
                return Note::A3;
            }
            58 => {
                return Note::A3Sharp;
            }
            59 => {
                return Note::B3;
            }
            60 => {
                return Note::C4;
            }
            61 => {
                return Note::C4Sharp;
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
            67 => {
                return Note::G4;
            }
            68 => {
                return Note::G4Sharp;
            }
            69 => {
                return Note::A4;
            }
            70 => {
                return Note::A4Sharp;
            }
            71 => {
                return Note::B4;
            }
            72 => {
                return Note::C5;
            }
            73 => {
                return Note::C5Sharp;
            }
            74 => {
                return Note::D5;
            }
            75 => {
                return Note::D5Sharp;
            }
            76 => {
                return Note::E5;
            }
            77 => {
                return Note::F5;
            }
            78 => {
                return Note::F5Sharp;
            }
            79 => {
                return Note::G5;
            }
            80 => {
                return Note::G5Sharp;
            }
            81 => {
                return Note::A5;
            }
            82 => {
                return Note::A5Sharp;
            }
            83 => {
                return Note::B5;
            }
            84 => {
                return Note::C6;
            }
            85 => {
                return Note::C6Sharp;
            }
            86 => {
                return Note::D6;
            }
            87 => {
                return Note::D6Sharp;
            }
            88 => {
                return Note::E6;
            }
            89 => {
                return Note::F6;
            }
            90 => {
                return Note::F6Sharp;
            }
            91 => {
                return Note::G6;
            }
            92 => {
                return Note::G6Sharp;
            }
            93 => {
                return Note::A6;
            }
            94 => {
                return Note::A6Sharp;
            }
            95 => {
                return Note::B6;
            }
            96 => {
                return Note::C7;
            }
            _ => {
                error!("Invalid byte: {:#?}", byte);
                return Note::InvalidByte;
            }
        }
    }
}
