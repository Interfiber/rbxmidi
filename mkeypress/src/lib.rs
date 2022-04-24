#![allow(non_upper_case_globals)]
#![allow(dead_code)]

pub const kVK_Return: u16 = 0x24;
pub const kVK_Tab: u16 = 0x30;
pub const kVK_Space: u16 = 0x31;
pub const kVK_Delete: u16 = 0x33;
pub const kVK_Escape: u16 = 0x35;
pub const kVK_Command: u16 = 0x37;
pub const kVK_Shift: u16 = 0x38;
pub const kVK_CapsLock: u16 = 0x39;
pub const kVK_Option: u16 = 0x3A;
pub const kVK_Control: u16 = 0x3B;
pub const kVK_RightShift: u16 = 0x3C;
pub const kVK_RightOption: u16 = 0x3D;
pub const kVK_RightControl: u16 = 0x3E;
pub const kVK_Function: u16 = 0x3F;
pub const kVK_F17: u16 = 0x40;
pub const kVK_VolumeUp: u16 = 0x48;
pub const kVK_VolumeDown: u16 = 0x49;
pub const kVK_Mute: u16 = 0x4A;
pub const kVK_F18: u16 = 0x4F;
pub const kVK_F19: u16 = 0x50;
pub const kVK_F20: u16 = 0x5A;
pub const kVK_F5: u16 = 0x60;
pub const kVK_F6: u16 = 0x61;
pub const kVK_F7: u16 = 0x62;
pub const kVK_F3: u16 = 0x63;
pub const kVK_F8: u16 = 0x64;
pub const kVK_F9: u16 = 0x65;
pub const kVK_F11: u16 = 0x67;
pub const kVK_F13: u16 = 0x69;
pub const kVK_F16: u16 = 0x6A;
pub const kVK_F14: u16 = 0x6B;
pub const kVK_F10: u16 = 0x6D;
pub const kVK_F12: u16 = 0x6F;
pub const kVK_F15: u16 = 0x71;
pub const kVK_Help: u16 = 0x72;
pub const kVK_Home: u16 = 0x73;
pub const kVK_PageUp: u16 = 0x74;
pub const kVK_ForwardDelete: u16 = 0x75;
pub const kVK_F4: u16 = 0x76;
pub const kVK_End: u16 = 0x77;
pub const kVK_F2: u16 = 0x78;
pub const kVK_PageDown: u16 = 0x79;
pub const kVK_F1: u16 = 0x7A;
pub const kVK_LeftArrow: u16 = 0x7B;
pub const kVK_RightArrow: u16 = 0x7C;
pub const kVK_DownArrow: u16 = 0x7D;
pub const kVK_UpArrow: u16 = 0x7E;
pub const kVK_ANSI_Keypad0: u16 = 0x52;
pub const kVK_ANSI_Keypad1: u16 = 0x53;
pub const kVK_ANSI_Keypad2: u16 = 0x54;
pub const kVK_ANSI_Keypad3: u16 = 0x55;
pub const kVK_ANSI_Keypad4: u16 = 0x56;
pub const kVK_ANSI_Keypad5: u16 = 0x57;
pub const kVK_ANSI_Keypad6: u16 = 0x58;
pub const kVK_ANSI_Keypad7: u16 = 0x59;
pub const kVK_ANSI_Keypad8: u16 = 0x5B;
pub const kVK_ANSI_Keypad9: u16 = 0x5C;
pub const kVK_ANSI_KeypadClear: u16 = 0x47;
pub const kVK_ANSI_KeypadDecimal: u16 = 0x41;
pub const kVK_ANSI_KeypadMultiply: u16 = 0x43;
pub const kVK_ANSI_KeypadPlus: u16 = 0x45;
pub const kVK_ANSI_KeypadDivide: u16 = 0x4B;
pub const kVK_ANSI_KeypadEnter: u16 = 0x4C;
pub const kVK_ANSI_KeypadMinus: u16 = 0x4E;
pub const kVK_ANSI_KeypadEquals: u16 = 0x51;
pub const kVK_RIGHT_COMMAND: u16 = 0x36;
pub const kVK_ANSI_A                 : u16 = 0x00;
pub const kVK_ANSI_S                 : u16 = 0x01;
pub const kVK_ANSI_D                 : u16 = 0x02;
pub const kVK_ANSI_F                 : u16 = 0x03;
pub const kVK_ANSI_H                 : u16 = 0x04;
pub const kVK_ANSI_G                 : u16 = 0x05;
pub const kVK_ANSI_Z                 : u16 = 0x06;
pub const kVK_ANSI_X                 : u16 = 0x07;
pub const kVK_ANSI_C                 : u16 = 0x08;
pub const kVK_ANSI_V                 : u16 = 0x09;
pub const kVK_ANSI_B                 : u16 = 0x0B;
pub const kVK_ANSI_Q                 : u16 = 0x0C;
pub const kVK_ANSI_W                 : u16 = 0x0D;
pub const kVK_ANSI_E                 : u16 = 0x0E;
pub const kVK_ANSI_R                 : u16 = 0x0F;
pub const kVK_ANSI_Y                 : u16 = 0x10;
pub const kVK_ANSI_T                 : u16 = 0x11;
pub const kVK_ANSI_1                 : u16 = 0x12;
pub const kVK_ANSI_2                 : u16 = 0x13;
pub const kVK_ANSI_3                 : u16 = 0x14;
pub const kVK_ANSI_4                 : u16 = 0x15;
pub const kVK_ANSI_6                 : u16 = 0x16;
pub const kVK_ANSI_5                 : u16 = 0x17;
pub const kVK_ANSI_Equal             : u16 = 0x18;
pub const kVK_ANSI_9                 : u16 = 0x19;
pub const kVK_ANSI_7                 : u16 = 0x1A;
pub const kVK_ANSI_Minus             : u16 = 0x1B;
pub const kVK_ANSI_8                 : u16 = 0x1C;
pub const kVK_ANSI_0                 : u16 = 0x1D;
pub const kVK_ANSI_RightBracket      : u16 = 0x1E;
pub const kVK_ANSI_O                 : u16 = 0x1F;
pub const kVK_ANSI_U                 : u16 = 0x20;
pub const kVK_ANSI_LeftBracket       : u16 = 0x21;
pub const kVK_ANSI_I                 : u16 = 0x22;
pub const kVK_ANSI_P                 : u16 = 0x23;
pub const kVK_ANSI_L                 : u16 = 0x25;
pub const kVK_ANSI_J                 : u16 = 0x26;
pub const kVK_ANSI_Quote             : u16 = 0x27;
pub const kVK_ANSI_K                 : u16 = 0x28;
pub const kVK_ANSI_Semicolon         : u16 = 0x29;
pub const kVK_ANSI_Backslash         : u16 = 0x2A;
pub const kVK_ANSI_Comma             : u16 = 0x2B;
pub const kVK_ANSI_Slash             : u16 = 0x2C;
pub const kVK_ANSI_N                 : u16 = 0x2D;
pub const kVK_ANSI_M                 : u16 = 0x2E;
pub const kVK_ANSI_Period            : u16 = 0x2F;
pub const kVK_ANSI_Grave             : u16 = 0x32;

extern "C" {
    fn test();
    fn sendkey(letter: u16, uppercase: bool);
}

pub fn testit(){
    unsafe {
        test();
    }
}

pub fn send_key(letter: u16){
    unsafe {
        sendkey(letter, false);
    }
}

pub fn send_key_shift(letter: u16){
    unsafe {
        sendkey(letter, true);
    }
}

pub fn send_key_wrap(letter: char){
    let keycode = key_to_int(letter);
    if letter.is_uppercase(){
        send_key_shift(keycode);
    } else {
        if letter == '!' || letter == '@' || letter == '#' || letter == '$' || letter == '%' || letter == '^' || letter == '&' || letter == '*' || letter == '(' || letter == ')' {
            send_key_shift(keycode);
        } else {
            send_key(keycode);
        }
    }
}

pub fn key_to_int(letter: char) -> u16 {
    let letter_lower = letter.to_ascii_lowercase();
    match letter_lower {
        '1' => {
            return kVK_ANSI_1;
        },
        '2' => {
            return kVK_ANSI_2;
        }
        '3' => {
            return kVK_ANSI_3;
        }
        '4' => {
            return kVK_ANSI_4;
        }
        '5' => {
            return kVK_ANSI_5;
        }
        '6' => {
            return kVK_ANSI_6;
        }
        '7' => {
            return kVK_ANSI_7;
        }
        '8' => {
            return kVK_ANSI_8;
        }
        '9' => {
            return kVK_ANSI_9;
        }
        '0' => {
            return kVK_ANSI_9;
        }
        '!' => {
            return kVK_ANSI_1;
        }
        '@' => {
            return kVK_ANSI_2;
        }
        '$' => {
            return kVK_ANSI_4;
        }
        '%' => {
            return kVK_ANSI_5;
        }
        '^' => {
            return kVK_ANSI_6;
        }
        '*' => {
            return kVK_ANSI_8;
        }
        '(' => {
            return kVK_ANSI_9;
        }
        'q' => {
            return kVK_ANSI_Q;
        }
        'e' => {
            return kVK_ANSI_E;
        }
        'r' => {
            return kVK_ANSI_R;
        }
        't' => {
            return kVK_ANSI_T;
        }
        'y' => {
            return kVK_ANSI_Y;
        }
        'u' => {
            return kVK_ANSI_U;
        }
        'i' => {
            return kVK_ANSI_I;
        }
        'o' => {
            return kVK_ANSI_O;
        }
        'O' => {
            return kVK_ANSI_O;
        }
        'p' => {
            return kVK_ANSI_P;
        }
        'a' => {
            return kVK_ANSI_A;
        }
        's' => {
            return kVK_ANSI_S;
        }
        'd' => {
           return kVK_ANSI_D; 
        }
        'f' => {
            return kVK_ANSI_F;
        }
        'g' => {
           return kVK_ANSI_G; 
        }
        'h' => {
            return kVK_ANSI_H;
        }
        'j' => {
            return kVK_ANSI_J;
        }
        'k' => {
            return kVK_ANSI_K;
        }
        'l' => {
            return kVK_ANSI_L;
        }
        'w' => {
            return kVK_ANSI_W;
        }
        'z' => {
            return kVK_ANSI_Z;
        }
        'x' => {
            return kVK_ANSI_X;
        }
        'c' => {
            return kVK_ANSI_C;
        }
        'v' => {
           return kVK_ANSI_V; 
        }
        'b' => {
            return kVK_ANSI_B;
        }
        'n' => {
            return kVK_ANSI_N;
        }
        'm' => {
            return kVK_ANSI_M;
        }
        _ => panic!("Unsupported character")
    }
}