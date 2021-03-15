use bulletproofs::{
	r1cs::{LinearCombination, Prover, Verifier},
	PedersenGens,
};
use curve25519_dalek::scalar::Scalar;

pub trait Hasher {
	fn hash(&self, xl: Scalar, xr: Scalar) -> Scalar;
}
