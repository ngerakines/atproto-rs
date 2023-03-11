use jsonwebtoken::jwk::Jwk;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Handle {
    pub did: String,
    pub handle: String,
    pub keys: Vec<Jwk>,
    pub password: String,
}
