use serde::{Serialize, Deserialize};

const CMD_INDEX: u8 = 1;
const CMD_POST_RECORD: u8 = 100;
const CMD_GET_RECORD: u8 = 120;

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub id: u8,
    pub data_len: u64,
    pub timestamp: u64,
}

impl Command {
    pub fn index() -> Command {
        Command {
            id: CMD_INDEX,
            data_len: 0,
            timestamp: 0
        }
    }

    pub fn post_record() -> Command {
        Command {
            id: CMD_POST_RECORD,
            data_len: 0,
            timestamp: 0
        }
    }

    pub fn get_record() -> Command {
        Command {
            id: CMD_GET_RECORD,
            data_len: 0,
            timestamp: 0
        }
    }
}
