use arrow_serde::packets::handshake::serverbound::Handshake;
use arrow_serde::Deserializer;
use serde::Deserialize;

use std::io::Cursor;

#[test]
pub fn test_handshake_deserialize() {
    let bytes: Vec<u8> = vec![47, 9, 49, 50, 55, 46, 48, 46, 48, 46, 49, 99, 221, 2];
    let deserializer = Deserializer {
        reader: Cursor::new(bytes),
    };
    let handshake = Handshake::deserialize(deserializer).unwrap();
}
