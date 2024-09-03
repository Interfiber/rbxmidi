use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::midi::data::Note;

/**
 * Config loaded from Library/RobloxMidi/Config.ron
 */
#[derive(Debug, Deserialize, Serialize)]
struct RobloxMidiConfig {
    pub author: String,
    pub keys: HashMap<Note, String>
}