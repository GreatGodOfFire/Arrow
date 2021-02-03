use std::io::Cursor;

use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use serde::{Deserialize, Deserializer as SDeserializer};

use arrow_serde::read_varint;
use arrow_serde::Deserializer;
// use arrow_serde::handshake::serverbound::Handshake;

use tokio::io::stdout;

use serde::de::SeqAccess;

use arrow_serde::packets::handshake::serverbound::Handshake;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = TcpListener::bind("0.0.0.0:25565").await?;
    let (mut socket, _) = listener.accept().await?;

    tokio::spawn(async move {
        loop {
            let mut buf = [0; 1024];

            match socket.read(&mut buf).await {
                Ok(n) if n == 0 => continue,
                Ok(_) => {}
                Err(e) => {
                    eprintln!("failed to read err = {}", e);
                    return;
                }
            };

            let mut cursor = Cursor::new(buf.to_vec());
            let len = read_varint(&mut cursor);
            let id = read_varint(&mut cursor);
            println!("len: {}\nid: {}", len.0, id.0);

            let mut bytes = vec![0; len.0 as usize - id.len()];

            cursor.read_exact(bytes.as_mut_slice()).await.unwrap();

            let cursor = Cursor::new(bytes);

            println!("{:?}", cursor);
            println!(
                "{:?}",
                Handshake::deserialize(&mut Deserializer { reader: cursor })
            );
            break;
        }
    })
    .await?;

    Ok(())
}

fn deserialize_packet<'a, D>(reader: Cursor<Vec<u8>>) -> D
where
    D: Deserialize<'a>,
{
    D::deserialize(&mut Deserializer { reader }).unwrap()
}
