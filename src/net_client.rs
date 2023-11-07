use std::io::{prelude::*, BufReader};
use std::net::TcpStream;


pub struct NetClient {
    pub connection: Option<TcpStream>
}

impl NetClient {
    pub fn get_record() -> Option<Vec<i16>> {
        match TcpStream::connect("127.0.0.1:33666") {
            Err(msg) => {
                println!("{:}", msg);
                None
            },

            Ok(mut connection) => {
                let command: [u8; 8] = [102, 0, 0, 0, 0, 0, 0, 0];

                connection.write(&command).unwrap();
                connection.flush().unwrap();

                let mut record_buf: Vec<u8> = Vec::new();
                let mut l: usize = 0;
                let mut read = true;

                let mut reader = BufReader::new(connection);

                reader.read_to_end(&mut record_buf).unwrap();

                println!("Done read. {:}", record_buf.len());

                let x: u64 = bincode::deserialize(&record_buf).unwrap();
                println!("Test serde len. {:}", x);

                let su = bincode::deserialize::<Vec<i16>>(&record_buf).unwrap();
                println!("Result len {:}", su.len());

                return Some(su);
            }
        }
    }

    pub fn post_record(data: &Vec<i16>) {
        match TcpStream::connect("127.0.0.1:33666") {
            Err(msg) => println!("{:}", msg),

            Ok(mut connection) => {
                let buf = bincode::serialize(data).unwrap();
                let length = buf.len();

                connection.write(bincode::serialize(&length).unwrap().as_slice()).unwrap();
                connection.write_all(&buf).unwrap();
                connection.flush().unwrap();
            }
        }
    }
}
