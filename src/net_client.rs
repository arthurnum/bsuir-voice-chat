use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

use crate::command::Command;
use crate::utils;
use crate::voice_list_item::VoiceListItem;


#[derive(Debug)]
pub struct NetClient {
    pub server_id: String,
}

impl NetClient {
    pub fn new() -> NetClient {
        let mut config = BufReader::new(
            File::open("config").unwrap()
        );
        let mut ip_str = String::new();
        config.read_line(&mut ip_str).unwrap();
        ip_str = format!("{:}:33666", ip_str.trim());

        NetClient { server_id: ip_str }
    }

    pub fn index(&self) -> Option<Vec<VoiceListItem>> {
        match TcpStream::connect(&self.server_id) {
            Err(msg) => {
                println!("{:}", msg);
                None
            },

            Ok(mut connection) => {
                let cmd = Command::index();

                connection.write(bincode::serialize(&cmd).unwrap().as_slice()).unwrap();
                connection.flush().unwrap();

                let mut buf: Vec<u8> = Vec::new();
                connection.read_to_end(&mut buf).unwrap();

                let records_list: Vec<VoiceListItem> = bincode::deserialize(&buf).unwrap();
                Some(records_list)
            }
        }
    }

    pub fn get_record(&self, id: u64) -> Option<Vec<i16>> {
        match TcpStream::connect(&self.server_id) {
            Err(msg) => {
                println!("{:}", msg);
                None
            },

            Ok(mut connection) => {
                let mut cmd = Command::get_record();
                cmd.timestamp = id;

                connection.write(bincode::serialize(&cmd).unwrap().as_slice()).unwrap();

                let mut record_buf: Vec<u8> = Vec::new();
                connection.read_to_end(&mut record_buf).unwrap();
                println!("Done read. {:}", record_buf.len());

                let x: u64 = bincode::deserialize(&record_buf).unwrap();
                println!("Test serde len. {:}", x);

                let su = bincode::deserialize::<Vec<i16>>(&record_buf).unwrap();
                println!("Result len {:}", su.len());

                Some(su)
            }
        }
    }

    pub fn post_record(&self, data: &Vec<i16>) {
        match TcpStream::connect(&self.server_id) {
            Err(msg) => println!("{:}", msg),

            Ok(mut connection) => {
                let mut cmd = Command::post_record();
                let buf = bincode::serialize(data).unwrap();

                cmd.data_len = buf.len() as u64;
                cmd.timestamp = utils::get_timestamp();

                connection.write(bincode::serialize(&cmd).unwrap().as_slice()).unwrap();
                connection.write_all(&buf).unwrap();
                connection.flush().unwrap();
            }
        }
    }
}
