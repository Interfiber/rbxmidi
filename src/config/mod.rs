use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::midi::data::Note;

/**
 * Config loaded from Library/RobloxMidi/Config.ron
 */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RobloxMidiConfig {
    pub author: String,
    pub keys: HashMap<Note, String>
}

impl RobloxMidiConfig {
    pub fn load(path: &str) -> Self {
       return ron::from_str(&std::fs::read_to_string(path).expect("Failed to read from config")).expect("Failed to load config");
    }
}