pub mod handshake;
pub mod login;

pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}
