use base64::{engine::general_purpose::STANDARD_NO_PAD as BASE64, Engine};
use rand::rngs::ThreadRng;
use rsa::{pkcs8::DecodePublicKey, Pkcs1v15Encrypt, RsaPublicKey};

use crate::{client_error, error::TgError};

pub(crate) struct Crypt {
    public_key: Option<RsaPublicKey>,
    rng: ThreadRng,
}

unsafe impl Send for Crypt {}

impl Crypt {
    fn new(public_key: Option<RsaPublicKey>) -> Self {
        Crypt {
            public_key,
            rng: rand::thread_rng(),
        }
    }

    pub(crate) fn from_pem(pem: Option<String>) -> Result<Self, TgError> {
        if let Some(pem) = pem {
            let pem = pem::parse(pem).map_err(|e| client_error!("pem parse error", e))?;
            let public_key = RsaPublicKey::from_public_key_der(pem.contents())
                .map_err(|e| client_error!("from_public_key_der error", e))?;

            Ok(Crypt::new(Some(public_key)))
        } else {
            Ok(Crypt::new(None))
        }
    }

    pub(crate) fn has_public_key(&self) -> bool {
        self.public_key.is_some()
    }

    pub(crate) fn encrypt(&mut self, plain_text: &str) -> Result<Option<String>, TgError> {
        if let Some(public_key) = &self.public_key {
            let encrypted = public_key
                .encrypt(&mut self.rng, Pkcs1v15Encrypt, plain_text.as_bytes())
                .map_err(|e| client_error!("encrypt error", e))?;
            let encoded = BASE64.encode(encrypted);
            Ok(Some(encoded))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encrypt() {
        let pem = "-----BEGIN PUBLIC KEY-----
    MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAv1bUBjEAu9hysYKYbYTS6uWjxsk3JTU/YES2p6n1weRl4MC9SDW5UVLP9FRIG8rl+hy8IUFMiw0hTxevHveZX/FVvfpKt2MLFywmfQG63b3VZplHmTlfB85h5gr8X4nvAL4kATl5NdIqM6zbjxJ6lHTLFipXENLEcfABKJQcDZdmD2+FTz0Vm+6PddggC3OREhE0hqw3G+sS6J1gsmP2tFAt0gZoh2JKeLpTgfT4zCmTDrRa5srVt0gN79EULFSVPo8zRsDIDxY653vhG1diZD3Z3g8KLwUsa21jCxHdW4ul6cAoSYrsON/uEFtcWl4nIrdpz5XQCbm6BOcXmO2yuwIDAQAB
    -----END PUBLIC KEY-----";

        let mut crypt = Crypt::from_pem(Some(pem.to_string())).unwrap();
        let _ = crypt.encrypt("test").unwrap();
    }
}
