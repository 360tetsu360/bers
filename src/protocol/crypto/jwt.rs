use serde::{Deserialize, Serialize};
use serde_json::json;

use super::{
    ecdsa::{KeyPair, PublicKey},
    error::CryptErr,
};

macro_rules! expect_two {
    ($iter:expr) => {{
        let mut i = $iter;
        match (i.next(), i.next(), i.next()) {
            (Some(first), Some(second), None) => (first, second),
            _ => return Err(CryptErr::JwtDecodingError),
        }
    }};
}

fn decode_b64(encoded: &str) -> Result<Vec<u8>, CryptErr> {
    match base64::decode_config(encoded, base64::URL_SAFE_NO_PAD) {
        Ok(p) => Ok(p),
        Err(e) => Err(CryptErr::Base64Error(e)),
    }
}

fn encode_b64(msg: &[u8]) -> String {
    base64::encode_config(msg, base64::URL_SAFE_NO_PAD)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
    pub alg: String,
    pub x5u: String,
}
impl Header {
    pub fn decode_header(jwt: &str) -> Result<Self, CryptErr> {
        let split = jwt.split('.').collect::<Vec<&str>>();
        let dheader = decode_b64(split[0])?;
        let header: Header = match serde_json::from_slice(&dheader) {
            Ok(p) => p,
            Err(e) => return Err(CryptErr::SerdeError(e)),
        };
        Ok(header)
    }
}

pub struct Jwt {
    // only ES384
    pub header: Header,
    pub payload: String, //not encoded
}

impl Jwt {
    pub fn decode(jwt: &str, key: &PublicKey) -> Result<Self, CryptErr> {
        let (signature, message) = expect_two!(jwt.rsplitn(2, '.'));
        let (claims, header) = expect_two!(message.rsplitn(2, '.'));

        let dheader = decode_b64(header)?;
        let header: Header = match serde_json::from_slice(&dheader) {
            Ok(p) => p,
            Err(e) => return Err(CryptErr::SerdeError(e)),
        };

        if header.alg != "ES384" {
            return Err(CryptErr::JwtDecodingError);
        }

        if !key.verify(&decode_b64(signature)?, message.as_bytes())? {
            return Err(CryptErr::JwtVerifyError);
        }

        Ok(Self {
            header,
            payload: match String::from_utf8(decode_b64(claims)?) {
                Ok(p) => p,
                Err(e) => return Err(CryptErr::Other(e.to_string())),
            },
        })
    }

    pub fn encode(payload: String, sign_key: &KeyPair) -> Result<String, CryptErr> {
        let header = json!(
            {
                "alg" : "ES384",
                "x5u" : sign_key.export_public_key()?
            }
        );
        let header_str = match serde_json::to_string(&header) {
            Ok(p) => p,
            Err(e) => return Err(CryptErr::SerdeError(e)),
        };

        let message = [
            encode_b64(header_str.as_bytes()),
            encode_b64(payload.as_bytes()),
        ]
        .join(".");
        let signature = encode_b64(&sign_key.sign(message.as_bytes())?);

        let jwt = [message, signature].join(".");

        Ok(jwt)
    }
}

#[test]
fn jwt() {
    let data = json!({
        "myname" : "tetsu360",
        "kk" : "a",
    });
    let payload = serde_json::to_string(&data).unwrap();

    let key_pair = KeyPair::gen();
    let jwt = Jwt::encode(payload.clone(), &key_pair).unwrap();

    let decoded = Jwt::decode(&jwt, &key_pair.public_key()).unwrap();

    assert_eq!(&payload, &decoded.payload);
}
