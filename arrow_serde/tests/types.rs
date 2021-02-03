use arrow_serde::types::*;
use std::io::Cursor;

#[test]
pub fn test_varint() {
    read_varint(&mut Cursor::new(vec![47]));
}
