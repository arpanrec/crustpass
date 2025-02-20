mod libsql_store;

use crate::{
    encryption::{decryption, encryption, generate_key},
    physical::libsql_store::LibSQLPhysical,
};
use sha2::{Digest, Sha256};
use std::fmt::Display;

#[derive(Clone, Debug)]
pub(crate) enum Physical {
    LibSQL(LibSQLPhysical),
}

#[derive(Debug)]
pub(crate) enum PhysicalError {
    Error(String),
    LibSQLError(String),
}

impl Display for PhysicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PhysicalError::Error(msg) => write!(f, "PhysicalError: {}", msg),
            PhysicalError::LibSQLError(msg) => write!(f, "LibSQLError: {}", msg),
        }
    }
}

impl Physical {
    pub(crate) async fn new(physical: crate::configuration::Physical) -> Self {
        match physical.physical_type.as_str() {
            "libsql" => Physical::LibSQL(LibSQLPhysical::new(physical).await),
            _ => panic!("Unsupported storage type"),
        }
    }
    pub(crate) async fn read_encrypted(
        &mut self,
        key: &str,
    ) -> Result<Option<(String, String)>, PhysicalError> {
        match self {
            Physical::LibSQL(physical_impl) => Ok(physical_impl.read(key).await?),
        }
    }
    pub(crate) async fn read(
        &mut self,
        key: &str,
        master_enc_key: (&str, &str),
    ) -> Result<Option<String>, PhysicalError> {
        let (encrypted_value, encryption_key_hash) = match self.read_encrypted(key).await? {
            Some(value) => value,
            None => return Ok(None),
        };
        let encrypted_encryption_key = match self {
            Physical::LibSQL(physical_impl) => {
                physical_impl.get_encryption_key(encryption_key_hash.as_str()).await?
            }
        };
        let encryption_key =
            decryption(master_enc_key.0, encrypted_encryption_key.as_str()).await.map_err(
                |ex| PhysicalError::Error(format!("Error decrypting encryption key: {}", ex)),
            )?;
        let value = decryption(encryption_key.as_str(), encrypted_value.as_str())
            .await
            .map_err(|ex| PhysicalError::Error(format!("Error decrypting value: {}", ex)))?;
        Ok(Some(value))
    }

    pub(crate) async fn write(
        &mut self,
        key: &str,
        value: &str,
        master_enc_key: (&str, &str),
    ) -> Result<(), PhysicalError> {
        let encryption_key = generate_key().await;
        let mut hasher = Sha256::new();
        hasher.update(encryption_key.as_bytes());
        let encryption_key_hash = hex::encode(hasher.finalize());
        let encryption_key_encrypted =
            encryption(master_enc_key.0, encryption_key.as_str()).await.map_err(|ex| {
                PhysicalError::Error(format!("Error encrypting encryption key: {}", ex))
            })?;
        let encrypted_value = encryption(encryption_key.as_str(), value)
            .await
            .map_err(|ex| PhysicalError::Error(format!("Error encrypting value: {}", ex)))?;

        match self {
            Physical::LibSQL(physical_impl) => {
                physical_impl
                    .write_encryption_key(
                        &encryption_key_encrypted,
                        &encryption_key_hash,
                        master_enc_key.1,
                    )
                    .await?;
                physical_impl.write(key, &encrypted_value, encryption_key_hash.as_str()).await
            }
        }
    }

    pub(crate) async fn delete(&mut self, key: &str) -> Result<(), PhysicalError> {
        match self {
            Physical::LibSQL(physical_impl) => Ok(physical_impl.delete(key).await?),
        }
    }
}
