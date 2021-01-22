pub mod serverbound {
    use arrow_serde_macros::Packet;

    #[derive(Debug, Packet)]
    pub struct LoginStart {
        name: String,
    }
}

pub mod clientbound {
    use arrow_serde_macros::Packet;

    #[derive(Debug, Packet)]
    pub struct LoginSuccess {
        uuid: String,
        username: String,
    }
}
