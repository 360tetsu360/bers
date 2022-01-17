use crate::protocol::crypto::cipher::Cipher;
use crate::protocol::crypto::ecdsa::{KeyPair, PublicKey};
use crate::protocol::crypto::error::CryptErr;
use crate::protocol::crypto::jwt::Jwt;

use rand::Rng;
use ring::digest;
use serde_json::json;

pub fn exchange(pubkey: PublicKey) -> Result<(String, Cipher), CryptErr> {
    //jwt and IV
    let keypair = KeyPair::gen();

    let shared_secret = keypair.ecdh(pubkey.bytes()).unwrap();

    let salt = rand::thread_rng().gen::<[u8; 16]>();

    let digest_alg = &digest::SHA256;
    let mut digest = digest::Context::new(digest_alg);
    digest.update(&salt);
    digest.update(&shared_secret);

    let secret_key = digest.finish();

    let claims = json!({
        "salt": base64::encode(&salt),
    });
    let claim_str = match serde_json::to_string(&claims) {
        Ok(p) => p,
        Err(e) => return Err(CryptErr::SerdeError(e)),
    };

    let jwt = Jwt::encode(claim_str, &keypair)?;

    let cipher = Cipher::new(secret_key.as_ref())?;
    Ok((jwt, cipher))
}
