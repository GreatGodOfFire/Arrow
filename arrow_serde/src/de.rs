use std::io::{Cursor, Read};

use serde::de::value::SeqDeserializer;

use crate::error::Error;
use crate::types;

pub struct Deserializer {
    pub reader: Cursor<Vec<u8>>,
}

impl Deserializer {
    fn read_byte(&mut self) -> Result<u8, Error> {
        let mut buf = [0u8];
        self.reader.read_exact(&mut buf).unwrap();
        Ok(buf[0])
    }
}

impl<'d> serde::Deserializer<'d> for Deserializer {
    type Error = Error;

    fn deserialize_any<V>(self, _: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        unimplemented!("deserialize_any not supported")
    }

    fn deserialize_bool<V>(mut self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_bool(match self.read_byte()? {
            0 => false,
            _ => true,
        })
    }

    fn deserialize_i8<V>(mut self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_i8(self.read_byte()? as i8)
    }

    fn deserialize_i16<V>(mut self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_i16(i16::from_be_bytes([self.read_byte()?, self.read_byte()?]))
    }

    fn deserialize_i32<V>(mut self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_i32(i32::from_be_bytes([
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
        ]))
    }

    fn deserialize_i64<V>(mut self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_i64(i64::from_be_bytes([
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
        ]))
    }

    fn deserialize_u8<V>(mut self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_u8(self.read_byte()?)
    }

    fn deserialize_u16<V>(mut self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_u16(u16::from_be_bytes([self.read_byte()?, self.read_byte()?]))
    }

    fn deserialize_u32<V>(mut self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_u32(u32::from_be_bytes([
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
        ]))
    }

    fn deserialize_u64<V>(mut self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_u64(u64::from_be_bytes([
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
        ]))
    }

    fn deserialize_f32<V>(mut self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_f32(f32::from_be_bytes([
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
        ]))
    }

    fn deserialize_f64<V>(mut self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_f64(f64::from_be_bytes([
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
            self.read_byte()?,
        ]))
    }

    fn deserialize_char<V>(self, _: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        self.deserialize_string(v)
    }

    fn deserialize_string<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        let mut reader = self.reader.clone();
        let len = types::read_varint(&mut reader);
        let mut buf = vec![0; len.0 as usize];
        reader.read_exact(buf.as_mut_slice()).unwrap();
        let string = String::from_utf8(buf).unwrap();
        v.visit_string(string)
    }

    fn deserialize_bytes<V>(self, _: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, _: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, _: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, _: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        _: &'static str,
        _: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        _: &'static str,
        _: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, v: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        v.visit_seq(SeqDeserializer::new(self.reader.into_inner().into_iter()))
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'d>,
    {
        todo!()
    }
}