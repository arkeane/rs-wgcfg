use curve25519_dalek::{edwards::EdwardsPoint, montgomery::MontgomeryPoint};
use rand_core::CryptoRng;
use rand_core::RngCore;

/*

Implementation of key pair creation on Curve25519 based on https://github.com/dalek-cryptography/x25519-dalek/

Copyright (c) 2017-2021 isis agora lovecruft. All rights reserved.
Copyright (c) 2019-2021 DebugSteven. All rights reserved.

*/

pub struct PrivKey(pub(crate) [u8; 32]);

impl PrivKey {
    pub fn new<T: RngCore + CryptoRng>(mut csrng: T) -> Self {
        let mut bytes = [0u8; 32];
        csrng.fill_bytes(&mut bytes);
        PrivKey(bytes)
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<Vec<u8>> for PrivKey {
    fn from(bytes: Vec<u8>) -> PrivKey {
        let mut arr = [0u8; 32];
        arr.copy_from_slice(&bytes[..32]);
        PrivKey(arr)
    }
}

impl AsRef<[u8]> for PrivKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

pub struct PubKey(pub(crate) MontgomeryPoint);

impl PubKey {
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }
}

impl From<[u8; 32]> for PubKey {
    fn from(bytes: [u8; 32]) -> PubKey {
        PubKey(MontgomeryPoint(bytes))
    }
}

impl<'a> From<&'a PrivKey> for PubKey {
    fn from(secret: &'a PrivKey) -> PubKey {
        PubKey(EdwardsPoint::mul_base_clamped(secret.0).to_montgomery())
    }
}

impl AsRef<[u8]> for PubKey {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Clone for PubKey {
    fn clone(&self) -> Self {
        PubKey(self.0)
    }
}
