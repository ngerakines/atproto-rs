fn main() {
    let jwks: jsonwebtoken::jwk::Jwk = serde_json::from_str(r#"{
        "kty": "EC",
        "d": "aYpmepsyNOU7vBZ-OXgttxgZNMlAK4iEEOWtabAGs9c",
        "use": "sig",
        "crv": "P-256",
        "kid": "sig-1678298459",
        "x": "D0Q3tu3LqSpBfR7-NH872dPRWLfv5OlhsXezziJsUk0",
        "y": "jizJqTAiV_c_Kvf0fwjL7KB1kzkYpIsXa6SPSFAfk5M",
        "alg": "ES256"
    }"#).unwrap();
    println!("{:?}", jwks)
}
