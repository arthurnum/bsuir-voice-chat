use std::io::prelude::*;
use std::net::TcpStream;

use crate::command::Command;
use crate::utils;
use crate::voice_list_item::VoiceListItem;

const SERVER_ADDR: &str = "127.0.0.1:33666";

pub struct NetClient {
    pub connection: Option<TcpStream>
}

impl NetClient {
    pub fn index() -> Option<Vec<VoiceListItem>> {
        match TcpStream::connect(SERVER_ADDR) {
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

    pub fn get_record(id: u64) -> Option<Vec<i16>> {
        match TcpStream::connect(SERVER_ADDR) {
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

    pub fn post_record(data: &Vec<i16>) {
        match TcpStream::connect(SERVER_ADDR) {
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
