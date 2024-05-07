extern crate bellman;
extern crate bls12_381;
extern crate ff;
extern crate rand;

use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bellman::groth16::{create_random_proof, prepare_verifying_key, verify_proof};
use bls12_381::{Bls12, Scalar};
use ff::Field;
use rand::thread_rng;

struct SquareRootCircuit {
    square_root: Option<Scalar>,
    public_input: Scalar,
}

impl Circuit<Bls12> for SquareRootCircuit {
    fn synthesize<CS: ConstraintSystem<Bls12>>(self, cs: &mut CS) -> Result<(), SynthesisError> {
        let square_root = cs.alloc(|| "square root", || self.square_root.ok_or(SynthesisError::AssignmentMissing))?;
        let public_input = cs.alloc_input(|| "public input", || Ok(self.public_input))?;

        cs.enforce(
            || "square root constraint",
            |lc| lc + square_root,
            |lc| lc + square_root,
            |lc| lc + public_input,
        );

        Ok(())
    }
}

fn main() {
    let params = Bls12::new();

    let secret = Scalar::from(3u64);
    let public_input = secret.square();

    let circuit = SquareRootCircuit {
        square_root: Some(secret),
        public_input,
    };

    let proof = create_random_proof(circuit, &params, &mut thread_rng()).unwrap();

    let pvk = prepare_verifying_key(&params.vk);
    assert!(verify_proof(&pvk, &proof, &[public_input]).unwrap());

    println!("Proof verified successfully!");
}
