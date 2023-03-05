#[derive(Clone, Debug)]
pub(crate) struct ServerConfig {
    pub version: String,
    pub port: u16,
    pub address: String,
}
