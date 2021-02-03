pub mod serverbound {
    use crate::types::Varint;

    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Handshake {
        version: Varint,
        string: String,
        port: u16,
        next_state: Varint,
    }

    // impl<'d> Deserialize<'d> for Handshake {
    //     fn deserialize<D>(d: D) -> Result<Self, D::Error>
    // where
    //     D: serde::Deserializer<'d> {
    //     // let d = &mut d;
    //     String::deserialize(d)?;

    //     Ok(Handshake {string: String::new()})
    // }
    // }

    // struct HandshakeVisitor;

    // impl<'v> Visitor<'v> for HandshakeVisitor {
    //     type Value = Handshake;

    //     fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    //         formatter.write_str("a handshake struct")
    //     }

    //     fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    //     where
    //             A: SeqAccess<'v>, {
    //                 // #(let #names = seq.next_element::<#types>()?.unwrap();)*
    //                 let string = seq.next_element::<String>().unwrap().unwrap();
    //                 Ok(Handshake { string })
    //     }
    // }

    #[derive(Deserialize)]
    pub struct LegacyServerListPing {
        pub payload: u8,
    }
}
