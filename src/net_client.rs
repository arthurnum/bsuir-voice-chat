use std::io::prelude::*;
use std::net::TcpStream;


pub struct NetClient {
    pub connection: Option<TcpStream>
}

impl NetClient {
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
