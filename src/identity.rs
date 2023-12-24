use std::{
	fmt,
	ops::{Deref, DerefMut},
};

use ed25519_dalek::{self, Signer};
use rand::{prelude::*, rngs::OsRng};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use zeroize::Zeroize;

use crate::common::*;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct PublicKey(ed25519_dalek::VerifyingKey);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Signature(ed25519_dalek::Signature);
pub type SignatureError = ed25519_dalek::SignatureError;

pub type Identity = PublicKey;
#[derive(Debug)]
pub struct PublicKeyError(ed25519_dalek::SignatureError);

#[derive(Debug, Serialize)]
pub struct PrivateKey {
	inner: ed25519_dalek::SigningKey,
	#[serde(skip_serializing)]
	copy: PrivateKeyCopy,
}
pub type KeypairError = ed25519_dalek::SignatureError;

#[derive(Debug, Zeroize)]
#[zeroize(drop)]
struct PrivateKeyCopy([u8; ed25519_dalek::SECRET_KEY_LENGTH]);

impl PublicKey {
	pub fn from_bytes(bytes: [u8; 32]) -> Result<Self, PublicKeyError> {
		Ok(Self(
			ed25519_dalek::VerifyingKey::from_bytes(&bytes).map_err(|e| PublicKeyError(e))?,
		))
	}

	pub fn generate_address(&self) -> IdType {
		let mut hasher = Sha256::new();
		hasher.update(self.0.to_bytes());
		let buffer: [u8; 32] = hasher.finalize().into();
		buffer.into()
	}

	pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
		self.0.verify_strict(message, &signature.0).is_ok()
	}
}

impl PrivateKey {
	pub fn as_bytes(&self) -> &[u8; 32] { &self.copy.0 }

	pub fn to_bytes(&self) -> [u8; 32] { self.inner.to_bytes() }

	pub fn from_bytes(mut bytes: [u8; 32]) -> Self {
		let this = Self::new(ed25519_dalek::SigningKey::from_bytes(&bytes));
		bytes.zeroize();
		this
	}

	pub fn generate() -> Self {
		let mut rng = OsRng {};
		Self::generate_with_rng(&mut rng)
	}

	pub fn generate_with_rng<R>(rng: &mut R) -> Self
	where
		R: CryptoRng + RngCore,
	{
		Self::new(ed25519_dalek::SigningKey::generate(rng))
	}

	fn new(inner: ed25519_dalek::SigningKey) -> Self {
		Self {
			copy: PrivateKeyCopy(inner.to_bytes()),
			inner,
		}
	}

	pub fn public(&self) -> PublicKey { PublicKey(self.inner.verifying_key()) }

	pub fn sign(&self, message: &[u8]) -> Signature { Signature(self.inner.sign(message)) }
}

impl FromSql for PrivateKey {
	fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
		match value {
			ValueRef::Blob(bytes) =>
				if bytes.len() >= ed25519_dalek::SECRET_KEY_LENGTH {
					FromSqlResult::Ok(PrivateKey::from_bytes(
						bytes[..ed25519_dalek::SECRET_KEY_LENGTH]
							.try_into()
							.unwrap(),
					))
				} else {
					FromSqlResult::Err(FromSqlError::InvalidBlobSize {
						expected_size: ed25519_dalek::SECRET_KEY_LENGTH,
						blob_size: bytes.len(),
					})
				},
			_ => FromSqlResult::Err(FromSqlError::InvalidType),
		}
	}
}

impl fmt::Display for PublicKeyError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
		write!(fmt, "{}", self.0)
	}
}

impl Clone for PrivateKey {
	fn clone(&self) -> Self {
		Self::new(ed25519_dalek::SigningKey::from_bytes(
			&self.inner.to_bytes(),
		))
	}
}

impl Signature {
	pub fn to_bytes(&self) -> [u8; 64] { self.0.to_bytes() }

	pub fn from_bytes(bytes: [u8; 64]) -> Self {
		Self(ed25519_dalek::Signature::from_bytes(&bytes))
	}

	pub fn hash(&self) -> IdType { IdType::hash(&self.to_bytes()) }
}

impl From<ed25519_dalek::VerifyingKey> for PublicKey {
	fn from(other: ed25519_dalek::VerifyingKey) -> Self { Self(other) }
}

impl Clone for PublicKey {
	fn clone(&self) -> Self {
		Self(ed25519_dalek::VerifyingKey::from_bytes(self.0.as_bytes()).unwrap())
	}
}

impl Deref for PublicKey {
	type Target = ed25519_dalek::VerifyingKey;

	fn deref(&self) -> &Self::Target { &self.0 }
}

impl DerefMut for PublicKey {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

#[cfg(test)]
mod tests {
	use rand::RngCore;

	use super::*;

	#[test]
	fn test_signature() {
		let mut buffer = vec![0u8; 1024];
		OsRng.fill_bytes(&mut buffer);

		let keypair = PrivateKey::generate();
		let signature = keypair.sign(&buffer);
		assert!(
			keypair.public().verify(&buffer, &signature),
			"can't verify own signature"
		);

		let signature_bytes = signature.to_bytes();
		let signature2 = Signature::from_bytes(signature_bytes);
		assert!(
			keypair.public().verify(&buffer, &signature2),
			"can't verify own signature after encoding+decoding it"
		);
	}
}
