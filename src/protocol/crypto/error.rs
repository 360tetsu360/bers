use std::fmt;

use static_dh_ecdh::CryptoError;

#[derive(Debug)]
pub enum CryptErr {
    SerdeError(serde_json::error::Error),
    CryptoError(CryptoError),
    Asn1DecodeError(simple_asn1::ASN1DecodeErr),
    Asn1EncodeError(simple_asn1::ASN1EncodeErr),
    Base64Error(base64::DecodeError),
    UnexceptedFormatError(String),
    KeyRejected(ring::error::KeyRejected),
    Unspecified(ring::error::Unspecified),
    JwtDecodingError,
    JwtVerifyError,
    BadPacket,
    Other(String),
}

impl fmt::Display for CryptErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CryptErr::SerdeError(e) => write!(f, "SerdeError {}", e),
            CryptErr::CryptoError(e) => write!(f, "CryptoError {}", e),
            CryptErr::Asn1DecodeError(e) => write!(f, "Asn1DecodeError: {}", e.to_string()),
            CryptErr::Asn1EncodeError(e) => write!(f, "Asn1EncodeError: {}", e.to_string()),
            CryptErr::Base64Error(e) => write!(f, "Base64Error: {}", e.to_string()),
            CryptErr::UnexceptedFormatError(str) => write!(f, "UnexceptedFormatError {}", str),
            CryptErr::KeyRejected(e) => write!(f, "KeyRejected {}", e.to_string()),
            CryptErr::Unspecified(e) => write!(f, "Unspecified {}", e.to_string()),
            CryptErr::JwtDecodingError => write!(f, "Jwt format error"),
            CryptErr::JwtVerifyError => write!(f, "Jwt verify error"),
            CryptErr::BadPacket => write!(f, "BadPacket"),
            CryptErr::Other(e) => write!(f, "Other {}", e),
        }
    }
}

impl std::error::Error for CryptErr {}
