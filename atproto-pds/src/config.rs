use jsonwebtoken::jwk::Jwk;

#[derive(Clone, Debug)]
pub(crate) struct ServerConfig {
    pub version: String,
    pub port: u16,
    pub address: String,
    pub signing_keys: Vec<Jwk>,
    pub default_signing_key: Jwk,
    pub handle_manager_type: String,
}
