use anyhow::Result;
use jsonwebtoken::jwk::Jwk;
use std::{fs::File, io::BufReader, path::Path};

pub fn jwk_from_file<P: AsRef<Path>>(path: P) -> Result<Jwk> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let document: Jwk = serde_json::from_reader(reader)?;
    Ok(document)
}

pub fn jwks_from(locations: Vec<String>) -> Result<Vec<Jwk>> {
    let mut jwks = Vec::new();
    for location in locations {
        let jwk = jwk_from_file(location)?;
        jwks.push(jwk);
    }
    Ok(jwks)
}
