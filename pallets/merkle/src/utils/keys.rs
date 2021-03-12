//! Type definitions used in merkle pallet

use sha2::Sha512;
use sp_std::prelude::*;

use codec::{Decode, Encode, EncodeLike, Input};
use curve25519_dalek::{
	ristretto::{CompressedRistretto, RistrettoPoint},
	scalar::Scalar,
};

#[derive(Eq, PartialEq, Clone, Default, Debug, Copy)]
pub struct Commitment(pub CompressedRistretto);
#[derive(Eq, PartialEq, Clone, Default, Debug, Copy)]
pub struct PrivateKey(pub Scalar);
#[derive(Eq, PartialEq, Clone, Default, Debug, Copy, Hash)]
pub struct ScalarData(pub Scalar);

pub const SIZE: usize = 32;

impl Encode for Commitment {
	fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
		(self.0).0.using_encoded(f)
	}
}

impl EncodeLike for Commitment {}

impl Decode for Commitment {
	fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
		match <[u8; SIZE] as Decode>::decode(input).map(CompressedRistretto) {
			Ok(elt) => Ok(Commitment(elt)),
			Err(e) => Err(e),
		}
	}
}

impl Encode for PrivateKey {
	fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
		(self.0).as_bytes().using_encoded(f)
	}
}

impl EncodeLike for PrivateKey {}

impl Decode for PrivateKey {
	fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
		match <[u8; SIZE] as Decode>::decode(input) {
			Ok(elt) => Ok(PrivateKey(Scalar::from_canonical_bytes(elt).unwrap_or(Scalar::zero()))),
			Err(e) => Err(e),
		}
	}
}

impl Commitment {
	/// Constructor from bytes
	pub fn new(bytes: &[u8]) -> Self {
		let point: RistrettoPoint = RistrettoPoint::hash_from_bytes::<Sha512>(bytes);
		Commitment(point.compress())
	}

	/// Serialize this public key to 32 bytes
	pub fn as_bytes(&self) -> Vec<u8> {
		(&self.0.as_bytes()).to_vec()
	}

	pub fn to_bytes(&self) -> Vec<u8> {
		self.as_bytes()
	}

	pub fn to_exact_bytes(&self) -> [u8; 32] {
		(self.0).0
	}

	// TODO: Make this more robust
	/// Deserialize this public key from 32 bytes
	pub fn from_bytes(bytes: &[u8]) -> Option<Commitment> {
		if bytes.len() != 32 {
			return None;
		}
		let mut arr = [0u8; 32];
		arr.copy_from_slice(bytes);
		let c = CompressedRistretto(arr);
		Some(Commitment(c))
	}

	pub fn from_ristretto(pt: RistrettoPoint) -> Self {
		Commitment(pt.compress())
	}

	pub fn hash_points(a: Self, b: Self) -> Self {
		Self::new(&[&a.0.to_bytes()[..], &b.0.to_bytes()[..]].concat()[..])
	}
}

impl Encode for ScalarData {
	fn using_encoded<R, F: FnOnce(&[u8]) -> R>(&self, f: F) -> R {
		(self.0).as_bytes().using_encoded(f)
	}
}

impl EncodeLike for ScalarData {}

impl Decode for ScalarData {
	fn decode<I: Input>(input: &mut I) -> Result<Self, codec::Error> {
		match <[u8; SIZE] as Decode>::decode(input) {
			Ok(elt) => Ok(ScalarData(Scalar::from_canonical_bytes(elt).unwrap_or(Scalar::zero()))),
			Err(e) => Err(e),
		}
	}
}

impl ScalarData {
	pub fn from(b: [u8; 32]) -> Self {
		ScalarData(Scalar::from_bytes_mod_order(b))
	}

	pub fn from_slice(b: &[u8]) -> Self {
		let mut buf: [u8; 32] = [0u8; 32];
		let length = if b.len() < 32 { b.len() } else { 32 };
		for i in 0..length {
			buf[i] = b[i]
		}
		Self::from(buf)
	}

	pub fn zero() -> Self {
		ScalarData(Scalar::zero())
	}

	pub fn to_scalar(&self) -> Scalar {
		self.0
	}
}
