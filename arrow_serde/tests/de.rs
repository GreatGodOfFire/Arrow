use arrow_serde::packets::handshake::serverbound::Handshake;
use arrow_serde::Deserializer;
use serde::Deserialize;

use std::io::Cursor;

#[test]
pub fn test_handshake_deserialize() {
    let bytes: Vec<u8> = vec![47, 9, 49, 50, 55, 46, 48, 46, 48, 46, 49, 99, 221, 2];
    let mut deserializer = Deserializer {
        reader: Cursor::new(bytes),
    };
    let handshake = Handshake::deserialize(&mut deserializer).unwrap();
}

#[derive(Deserialize, PartialEq, Debug)]
struct PrimitiveStruct {
    bool: bool,
    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
    u8: u8,
    u16: u16,
    u32: u32,
    u64: u64,
    f32: f32,
    f64: f64,
    string: String,
}

#[test]
pub fn test_deserialize_primitives() {
    let mut bytes: Vec<u8> = vec![];
    bytes.append(&mut vec![1]);
    bytes.append(&mut vec![2]);
    bytes.append(&mut 3i16.to_be_bytes().to_vec());
    bytes.append(&mut 4i32.to_be_bytes().to_vec());
    bytes.append(&mut 5i64.to_be_bytes().to_vec());
    bytes.append(&mut vec![6]);
    bytes.append(&mut 7u16.to_be_bytes().to_vec());
    bytes.append(&mut 8u32.to_be_bytes().to_vec());
    bytes.append(&mut 9u64.to_be_bytes().to_vec());
    bytes.append(&mut 1.0f32.to_be_bytes().to_vec());
    bytes.append(&mut 1.1f64.to_be_bytes().to_vec());
    bytes.append(&mut vec![5]);
    bytes.append(&mut "other".as_bytes().to_vec());

    let mut deserializer = Deserializer {
        reader: Cursor::new(bytes),
    };
    let primitive_struct = PrimitiveStruct::deserialize(&mut deserializer).unwrap();
    assert_eq!(
        PrimitiveStruct {
            bool: true,
            i8: 2,
            i16: 3i16,
            i32: 4i32,
            i64: 5i64,
            u8: 6,
            u16: 7u16,
            u32: 8u32,
            u64: 9u64,
            f32: 1.0f32,
            f64: 1.1f64,
            string: "other".to_string()
        },
        primitive_struct
    );
}
