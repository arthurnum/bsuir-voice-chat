use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceListItem {
    pub timestamp: u64,
}
