mod libsql_store;

use crate::{
    encryption::{decryption, encryption},
    physical::libsql_store::LibSQLPhysical,
};
use std::fmt::Display;
use tracing::warn;

#[derive(Clone, Debug)]
pub(crate) enum Physical {
    LibSQL(LibSQLPhysical),
}

#[derive(Debug)]
pub(crate) struct PhysicalError(String);

impl Display for PhysicalError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Physical Error: {}", self.0)
    }
}

impl Physical {
    pub(crate) async fn new(physical: crate::configuration::Physical) -> Self {
        match physical.physical_type.as_str() {
            "libsql" => Physical::LibSQL(LibSQLPhysical::new(physical).await),
            _ => panic!("Unsupported storage type"),
        }
    }

    pub(crate) async fn read(
        &mut self,
        key: &str,
        master_enc_key: (&str, &str),
        _: &str,
    ) -> Result<Option<String>, PhysicalError> {
        let result = match self {
            Physical::LibSQL(physical_impl) => physical_impl
                .read(key)
                .await
                .map_err(|ex| PhysicalError(format!("Error reading from libsql: {}", ex)))?,
        };

        if let Some((encrypted_value, key_hash)) = result {
            warn!("Not using key_hash: {}", key_hash);
            let value = decryption(master_enc_key.0, &encrypted_value)
                .await
                .map_err(|ex| PhysicalError(format!("Error decrypting value: {}", ex)))?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn write(
        &mut self,
        key: &str,
        value: &str,
        master_enc_key: (&str, &str),
        _: &str,
    ) -> Result<(), PhysicalError> {
        let encrypted_value = encryption(master_enc_key.0, value)
            .await
            .map_err(|ex| PhysicalError(format!("Error encrypting value: {}", ex)))?;

        match self {
            Physical::LibSQL(physical_impl) => physical_impl
                .write(key, &encrypted_value, master_enc_key.1)
                .await
                .map_err(|ex| PhysicalError(format!("Error writing to libsql: {}", ex))),
        }
    }

    pub(crate) async fn delete(&mut self, key: &str) -> Result<(), PhysicalError> {
        match self {
            Physical::LibSQL(physical_impl) => physical_impl
                .delete(key)
                .await
                .map_err(|ex| PhysicalError(format!("Error deleting from libsql: {}", ex))),
        }
    }
}
