use std::{collections::HashMap, path::Path};

use log::{error, info};
use serde::{Deserialize, Serialize};

use crate::midi::data::Note;

/**
 * Config loaded from Library/RobloxMidi/Config.ron
 */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RobloxMidiConfig {
    pub author: String,
    pub keys: HashMap<Note, String>,
}

impl RobloxMidiConfig {
    pub fn load(path: &str) -> Self {
        let paths = path.split(":");

        let mut good_path = String::new();

        for path in paths {
            if Path::exists(Path::new(path)) {
                good_path = path.to_string();

                info!("Picked Library location of: {}", path);
            }
        }

        if good_path.is_empty() {
            error!("No good library path found!");

            std::process::exit(-1);
        }

        return ron::from_str(&std::fs::read_to_string(good_path).expect("Failed to read from config"))
            .expect("Failed to load config");
    }
}
