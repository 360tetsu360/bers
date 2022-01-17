use super::error::CryptErr;
use rand::Rng;
use ring;
use static_dh_ecdh::ecdh::ecdh::{FromBytes, KeyExchange, PkP384, SkP384, ToBytes, ECDHNISTP384};

#[derive(Clone)]
pub struct KeyPair {
    private: SkP384,
    public: PkP384,
}

impl KeyPair {
    pub fn gen() -> Self {
        let random_seed = rand::thread_rng().gen::<[u8; 32]>();
        let private_key = ECDHNISTP384::<48>::generate_private_key(random_seed);
        let public_key = ECDHNISTP384::<48>::generate_public_key(&private_key);
        Self {
            private: private_key,
            public: public_key,
        }
    }

    pub fn private_key_bytes(&self) -> Vec<u8> {
        self.private.to_bytes().to_vec()
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.public.to_bytes().to_vec()
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey::from_bytes(&self.public.to_bytes())
    }

    pub fn export_public_key(&self) -> Result<String, CryptErr> {
        let bytes = self.public_key_bytes();
        let pubkey = PublicKey::from_bytes(&bytes);
        pubkey.to_pem()
    }

    pub fn ecdh(&self, pubkey: &[u8]) -> Result<Vec<u8>, CryptErr> {
        let peer_pubkey = match PkP384::from_bytes(pubkey) {
            Ok(p) => p,
            Err(e) => return Err(CryptErr::CryptoError(e)),
        };
        let shared_secret =
            ECDHNISTP384::<48>::generate_shared_secret(&self.private, &peer_pubkey).unwrap();
        Ok(shared_secret.to_bytes().to_vec())
    }

    pub fn ecdh_from_pem(&self, pubkey_pem: &str) -> Result<Vec<u8>, CryptErr> {
        let pubkey = PublicKey::from_pem(pubkey_pem)?;
        self.ecdh(pubkey.bytes())
    }

    pub fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, CryptErr> {
        let signing_algorithm = &ring::signature::ECDSA_P384_SHA384_FIXED_SIGNING;
        let keypair = match ring::signature::EcdsaKeyPair::from_private_key_and_public_key(
            signing_algorithm,
            &self.private.to_bytes(),
            &self.public.to_bytes(),
        ) {
            Ok(p) => p,
            Err(e) => return Err(CryptErr::KeyRejected(e)),
        };

        let rng = ring::rand::SystemRandom::new();
        match keypair.sign(&rng, msg) {
            Ok(a) => Ok(a.as_ref().to_vec()),
            Err(e) => Err(CryptErr::Unspecified(e)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub pubkey: Vec<u8>,
}

impl PublicKey {
    pub fn from_bytes(key: &[u8]) -> Self {
        Self {
            pubkey: key.to_vec(),
        }
    }

    pub fn from_pem(pem: &str) -> Result<Self, CryptErr> {
        let decode = match base64::decode(pem) {
            Ok(d) => d,
            Err(e) => {
                return Err(CryptErr::Base64Error(e));
            }
        };

        Self::from_der(&decode)
    }

    pub fn from_der(der: &[u8]) -> Result<Self, CryptErr> {
        let asn1 = match simple_asn1::from_der(der) {
            Ok(asn1) => asn1,
            Err(e) => {
                return Err(CryptErr::Asn1DecodeError(e));
            }
        };

        let body = match asn1.last() {
            Some(p) => p,
            None => {
                return Err(CryptErr::UnexceptedFormatError(
                    "Different format".to_owned(),
                ));
            }
        };

        let mut ret: Option<Self> = None;
        if let simple_asn1::ASN1Block::Sequence(_length, elems) = body {
            if elems.len() == 2 {
                let oids = elems.first().unwrap();
                if let simple_asn1::ASN1Block::Sequence(_length, oids) = oids {
                    if oids.len() == 2 {
                        if let simple_asn1::ASN1Block::ObjectIdentifier(_length, oid) =
                            oids.first().unwrap()
                        {
                            let ec = simple_asn1::oid!(1, 2, 840, 10_045, 2, 1);
                            if ec != *oid {
                                return Err(CryptErr::UnexceptedFormatError(
                                    "Key is not ec".to_owned(),
                                ));
                            }
                        }

                        if let simple_asn1::ASN1Block::ObjectIdentifier(_length, oid) =
                            oids.last().unwrap()
                        {
                            let p384 = simple_asn1::oid!(1, 3, 132, 0, 34);
                            if p384 != *oid {
                                return Err(CryptErr::UnexceptedFormatError(
                                    "Key is not secp384r1".to_owned(),
                                ));
                            }
                        }
                    }
                }

                let bit_string = elems.last().unwrap();
                if let simple_asn1::ASN1Block::BitString(_length, _bit_length, buff) = bit_string {
                    ret = Some(Self {
                        pubkey: buff.to_vec(),
                    });
                }
            }
        }
        match ret {
            Some(p) => Ok(p),
            None => Err(CryptErr::UnexceptedFormatError(
                "Different format".to_owned(),
            )),
        }
    }

    pub fn to_der(&self) -> Result<Vec<u8>, CryptErr> {
        let oid1 =
            simple_asn1::ASN1Block::ObjectIdentifier(0, simple_asn1::oid!(1, 2, 840, 10_045, 2, 1));
        let oid2 = simple_asn1::ASN1Block::ObjectIdentifier(0, simple_asn1::oid!(1, 3, 132, 0, 34));

        let oid_vec = simple_asn1::ASN1Block::Sequence(0, vec![oid1, oid2]);

        let pubkey =
            simple_asn1::ASN1Block::BitString(20, self.bytes().len() * 8, self.bytes().to_vec()); //bit

        let sequence = simple_asn1::ASN1Block::Sequence(0, vec![oid_vec, pubkey]);
        match simple_asn1::to_der(&sequence) {
            Ok(p) => Ok(p),
            Err(e) => Err(CryptErr::Asn1EncodeError(e)),
        }
    }

    pub fn to_pem(&self) -> Result<String, CryptErr> {
        match self.to_der() {
            Ok(der) => Ok(base64::encode(der)),
            Err(e) => Err(e),
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.pubkey
    }

    pub fn verify(&self, signature: &[u8], msg: &[u8]) -> Result<bool, CryptErr> {
        let valg = &ring::signature::ECDSA_P384_SHA384_FIXED;
        let verifier = ring::signature::UnparsedPublicKey::new(valg, &self.pubkey);
        match verifier.verify(msg, signature) {
            Ok(_) => Ok(true),
            Err(e) => Err(CryptErr::Unspecified(e)),
        }
    }
}

#[test]
fn test() {
    let data = b"Hello my name is tetsu36o!!!";
    let key_pair = KeyPair::gen();
    let signature = key_pair.sign(data).unwrap();

    let verify = key_pair.public_key().verify(&signature, data).unwrap();

    assert!(verify);
}
