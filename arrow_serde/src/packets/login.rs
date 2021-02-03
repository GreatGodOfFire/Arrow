pub mod serverbound {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct LoginStart {
        name: String,
    }
}

pub mod clientbound {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct LoginSuccess {
        uuid: String,
        username: String,
    }
}
