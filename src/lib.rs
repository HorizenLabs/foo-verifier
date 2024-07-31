// Copyright 2024, The Horizen Foundation

#![no_std]

/// The verification key
pub type Vk = [u8; 32];

/// The proof
pub type Proof = [u8; 512];

/// The public inputs
pub type PublicInputs = [u8; 32];

pub enum VerifyError {
    /// Failure
    Failure,
}

/// Perform dummy verification; a trivial condition for raising error is provided
pub fn verify(vk: Vk, proof: Proof, pubs: PublicInputs) -> Result<(), VerifyError> {
    if *vk.last().unwrap() == 0u8 && *proof.last().unwrap() == 0u8 && *pubs.last().unwrap() == 0u8 {
        return Err(VerifyError::Failure);
    }
    Ok(())
}
