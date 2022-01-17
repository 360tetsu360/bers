use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::protocol::{
    crypto::{
        ecdsa::PublicKey,
        error::CryptErr,
        jwt::{Header, Jwt},
    },
    types::player_data::{ExtraData, PlayerData},
};

#[derive(Serialize, Deserialize)]
pub struct Chain {
    chain: Vec<String>,
}

const MOJNG_PUBLIC_KEY : &str = "MHYwEAYHKoZIzj0CAQYFK4EEACIDYgAE8ELkixyLcwlZryUQcu1TvPOmI2B7vX83ndnWRUaXm74wFfa5f/lwQNTfrLVHa2PmenpGI6JhIMUJaWZrjmMj90NoKNFSNBuKdm8rYiXsfaz3K36x/1U26HpG0ZxK/V1V";

pub fn verify(chain: String) -> Result<(PublicKey, ExtraData), CryptErr> {
    let chain: Chain = match serde_json::from_str(&chain) {
        Ok(p) => p,
        Err(e) => return Err(CryptErr::SerdeError(e)),
    };

    let first_key_header = Header::decode_header(&chain.chain[0])?;

    let x5u = first_key_header.x5u;
    let mut pubkey = PublicKey::from_pem(&x5u)?;
    let mut verified = false;

    let mut final_key: Option<PublicKey> = None;

    let mut extra_data: Option<ExtraData> = None;

    for jwt in &chain.chain {
        let token = Jwt::decode(jwt, &pubkey)?;

        let x5u = Header::decode_header(jwt)?.x5u;

        if x5u == MOJNG_PUBLIC_KEY {
            verified = true;
        }

        let claims: Value = match serde_json::from_str(&token.payload) {
            Ok(p) => p,
            Err(e) => return Err(CryptErr::SerdeError(e)),
        };

        let identity_public_key = match claims.get("identityPublicKey") {
            Some(p) => p,
            None => {
                return Err(CryptErr::UnexceptedFormatError(
                    "no identityPublicKey".to_string(),
                ))
            }
        };

        let identity_public_key_str = match identity_public_key.as_str() {
            Some(p) => p,
            None => {
                return Err(CryptErr::UnexceptedFormatError(
                    "identityPublicKey is not string".to_string(),
                ))
            }
        };

        pubkey = PublicKey::from_pem(identity_public_key_str)?;

        if let Some(data) = claims.get("extraData") {
            extra_data = ExtraData::from_value(data);
        }

        final_key = Some(pubkey.clone());
    }

    if !verified {
        return Err(CryptErr::UnexceptedFormatError(
            "Unexcepted chain".to_owned(),
        ));
    }

    Ok((final_key.unwrap(), extra_data.unwrap()))
}

pub fn verify_skin(skin_jwt: String, pubkey: &PublicKey) -> Result<PlayerData, CryptErr> {
    let player_data_raw = Jwt::decode(&skin_jwt, pubkey)?;
    let player_data: PlayerData = match serde_json::from_str(&player_data_raw.payload) {
        Ok(p) => p,
        Err(e) => return Err(CryptErr::SerdeError(e)),
    };

    Ok(player_data)
}
