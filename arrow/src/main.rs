use std::io::Cursor;

use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;

use serde::{Deserialize, Deserializer as SDeserializer};

use arrow_serde::Deserializer;
use arrow_serde::read_varint;
// use arrow_serde::handshake::serverbound::Handshake;
use arrow_serde_macros::Packet;

use tokio::io::stdout;

use serde::de::SeqAccess;

use arrow_serde::packets::handshake::serverbound::Handshake;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut listener = TcpListener::bind("0.0.0.0:25565").await?;
    let (mut socket, _) = listener.accept().await?;

    // tokio::spawn(async move {
    //     let mut buf = [0; 1024];

    //     match socket.read(&mut buf).await {
    //         Ok(n) if n == 0 => return,
    //         Ok(_) => {},
    //         Err(e) => {
    //             eprintln!("failed to read err = {}", e);
    //             return;
    //         }
    //     };

    //     let mut vec = buf.to_vec();
    //     vec.reverse();
    //     vec.pop().unwrap();
    //     vec.pop().unwrap();
    //     vec.reverse();
    //     println!("{:?}", buf);

    //     // let bytes: Vec<u8> = vec![47, 9, 49, 50, 55, 46, 48, 46, 48, 46, 49, 99, 221, 2];
    //     let deserializer = Deserializer {
    //         reader: Cursor::new(vec),
    //     };
    //     let handshake = Handshake::deserialize(deserializer);
    //     println!("{:?}", handshake);
    // }).await.unwrap();

    // loop {

    //     

        tokio::spawn(async move {

            loop {
                let mut buf = [0; 1024];

                match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => continue,
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("failed to read err = {}", e);
                        return
                    }
                };

                let mut cursor = Cursor::new(buf.to_vec());
                let len = read_varint(&mut cursor);
                let id = read_varint(&mut cursor);
                let version = read_varint(&mut cursor);
                println!("len: {}\nid: {}\nversion: {}", len.0, id.0, version.0);
                
                // let mut packet_bytes = vec![47, 9, 49, 50, 55, 46, 48, 46, 48, 46, 49, 99, 221, 2];

                let mut bytes = vec![0; len.0 as usize];

                cursor.read_exact(bytes.as_mut_slice()).await.unwrap();

                // println!("{:?}", String::deserialize(Deserializer { reader: Cursor::new(bytes) }));
                
                // let bytes = vec![8, 100, 100, 100, 100, 100, 100, 100, 100];

                let cursor = Cursor::new(bytes);

                println!("{:?}", cursor);
                println!("{:?}", Handshake::deserialize(Deserializer { reader: cursor }));
                break;

                // let handshake = Handshake::deserialize();

                // println!("{:?}", cursor.clone().into_inner());
                // let handshake = deserialize_packet::<handshake::serverbound::Handshake>(Cursor::new(packet_bytes));
                // println!("{:?}", handshake);
            }
        }).await?;
    // }

    Ok(())
}

fn deserialize_packet<'a, D>(reader: Cursor<Vec<u8>>) -> D
where D: Deserialize<'a>, {
    D::deserialize(Deserializer { reader }).unwrap()
}