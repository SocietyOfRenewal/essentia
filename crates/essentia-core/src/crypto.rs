use std::convert::TryInto;

use anyhow::Result;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;
use serde::Serialize;

use crate::error::ChainError;
use crate::models::{Block, Did, KeyFile, SignedTransaction, Transaction, UnsignedBlock};

pub fn hash_bytes(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

pub fn hash_json<T: Serialize>(value: &T) -> Result<String> {
    Ok(hash_bytes(&serde_json::to_vec(value)?))
}

pub fn did_from_public_key(public_key_hex: &str) -> Did {
    format!("did:essentia:{}", public_key_hex.to_ascii_lowercase())
}

pub fn generate_key_file() -> KeyFile {
    let mut rng = OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();
    let public_key = hex::encode(verifying_key.to_bytes());
    let secret_key = hex::encode(signing_key.to_bytes());
    KeyFile {
        algorithm: "ed25519-prototype".to_string(),
        did: did_from_public_key(&public_key),
        public_key,
        secret_key,
    }
}

pub fn signing_key_from_key_file(key_file: &KeyFile) -> Result<SigningKey> {
    let secret: [u8; 32] = hex::decode(&key_file.secret_key)?
        .try_into()
        .map_err(|_| anyhow::anyhow!("secret key must be 32 bytes"))?;
    Ok(SigningKey::from_bytes(&secret))
}

pub fn verifying_key_from_hex(public_key_hex: &str) -> Result<VerifyingKey> {
    let public_key: [u8; 32] = hex::decode(public_key_hex)?
        .try_into()
        .map_err(|_| anyhow::anyhow!("public key must be 32 bytes"))?;
    Ok(VerifyingKey::from_bytes(&public_key)?)
}

pub fn transaction_signing_payload(tx: &Transaction, signer: &str) -> Result<Vec<u8>> {
    Ok(serde_json::to_vec(&(signer, tx))?)
}

pub fn sign_transaction(key_file: &KeyFile, tx: Transaction) -> Result<SignedTransaction> {
    let signing_key = signing_key_from_key_file(key_file)?;
    let payload = transaction_signing_payload(&tx, &key_file.did)?;
    let signature = signing_key.sign(&payload);
    let tx_hash = hash_bytes(&payload);
    Ok(SignedTransaction {
        tx,
        signer: key_file.did.clone(),
        public_key: key_file.public_key.clone(),
        signature: hex::encode(signature.to_bytes()),
        tx_hash,
    })
}

pub fn verify_signed_transaction(tx: &SignedTransaction) -> Result<()> {
    let expected_did = did_from_public_key(&tx.public_key);
    if tx.signer != expected_did {
        return Err(ChainError::InvalidDid(format!(
            "signer {} does not match public key did {}",
            tx.signer, expected_did
        ))
        .into());
    }
    let payload = transaction_signing_payload(&tx.tx, &tx.signer)?;
    let expected_hash = hash_bytes(&payload);
    if tx.tx_hash != expected_hash {
        return Err(ChainError::InvalidTransaction(format!(
            "tx_hash mismatch: expected {}, got {}",
            expected_hash, tx.tx_hash
        ))
        .into());
    }
    let verifying_key = verifying_key_from_hex(&tx.public_key)?;
    let signature_bytes: [u8; 64] = hex::decode(&tx.signature)?
        .try_into()
        .map_err(|_| anyhow::anyhow!("signature must be 64 bytes"))?;
    let signature = Signature::from_bytes(&signature_bytes);
    verifying_key.verify(&payload, &signature)?;
    Ok(())
}

pub fn block_signing_payload(block: &UnsignedBlock) -> Result<Vec<u8>> {
    Ok(serde_json::to_vec(block)?)
}

pub fn sign_block(key_file: &KeyFile, block: UnsignedBlock) -> Result<Block> {
    let signing_key = signing_key_from_key_file(key_file)?;
    if key_file.did != block.proposer {
        return Err(ChainError::Unauthorized(format!(
            "key {} cannot sign block for proposer {}",
            key_file.did, block.proposer
        ))
        .into());
    }
    let payload = block_signing_payload(&block)?;
    let signature = signing_key.sign(&payload);
    let block_hash = hash_bytes(&payload);
    Ok(Block {
        height: block.height,
        chain_id: block.chain_id,
        prev_hash: block.prev_hash,
        proposer: block.proposer,
        timestamp_unix_ms: block.timestamp_unix_ms,
        txs: block.txs,
        state_root: block.state_root,
        block_hash,
        signature: hex::encode(signature.to_bytes()),
    })
}

pub fn verify_block_signature(block: &Block, public_key_hex: &str) -> Result<()> {
    let unsigned = UnsignedBlock {
        height: block.height,
        chain_id: block.chain_id.clone(),
        prev_hash: block.prev_hash.clone(),
        proposer: block.proposer.clone(),
        timestamp_unix_ms: block.timestamp_unix_ms,
        txs: block.txs.clone(),
        state_root: block.state_root.clone(),
    };
    let payload = block_signing_payload(&unsigned)?;
    let expected_hash = hash_bytes(&payload);
    if block.block_hash != expected_hash {
        return Err(ChainError::Validation(format!(
            "block hash mismatch: expected {}, got {}",
            expected_hash, block.block_hash
        ))
        .into());
    }
    let verifying_key = verifying_key_from_hex(public_key_hex)?;
    let signature_bytes: [u8; 64] = hex::decode(&block.signature)?
        .try_into()
        .map_err(|_| anyhow::anyhow!("signature must be 64 bytes"))?;
    let signature = Signature::from_bytes(&signature_bytes);
    verifying_key.verify(&payload, &signature)?;
    Ok(())
}
