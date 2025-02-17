use libsql::{Builder, Connection};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};
// CREATE TABLE secrets_d (
//     id_d INTEGER PRIMARY KEY AUTOINCREMENT,
//     key_d TEXT NOT NULL,
//     value_d TEXT NOT NULL,
//     version_d INTEGER DEFAULT (1) NOT NULL,
//     updated_at_d INTEGER DEFAULT (-1) NOT NULL,
//     is_deleted_d INTEGER DEFAULT (0) NOT NULL
// );
// CREATE UNIQUE INDEX secrets_d_key_d_IDX ON secrets_d (key_d,version_d);

#[derive(Clone, Debug, Deserialize)]
struct LibSQLDetails {
    table_name: String,
    db_url: String,
    auth_token: String,
}

#[derive(Clone, Debug)]
pub struct LibSQLPhysical {
    libsql_details: LibSQLDetails,
}

pub enum LibSQLPhysicalError {
    ConnectionError(libsql::Error),
}

impl LibSQLPhysical {
    pub fn new(physical: crate::configuration::Physical) -> Self {
        if physical.physical_type != "libsql" {
            panic!("Only sqlite is supported at this time");
        }
        let libsql_details: LibSQLDetails = serde_json::from_value(physical.physical_details)
            .expect("Unable to parse storage_config");
        LibSQLPhysical { libsql_details }
    }

    async fn get_connection(&mut self) -> Result<Connection, LibSQLPhysicalError> {
        Builder::new_remote(
            self.libsql_details.db_url.clone(),
            self.libsql_details.auth_token.clone(),
        )
        .build()
        .await
        .map_err(|ex| LibSQLPhysicalError::ConnectionError(ex))?
        .connect()
        .map_err(|ex| LibSQLPhysicalError::ConnectionError(ex))
    }

    async fn get_current_version(&mut self, key: &str) -> Result<i64, LibSQLPhysicalError> {
        let table_name = self.libsql_details.table_name.clone();
        let mut rows = self
            .get_connection()
            .await?
            .query(
                &format!("SELECT version_d FROM {table_name} WHERE key_d = ? ORDER BY version_d DESC LIMIT 1;"),
                libsql::params![key],
            )
            .await
            .map_err(|ex| LibSQLPhysicalError::ConnectionError(ex))?;
        if let Some(row) =
            rows.next().await.map_err(|ex| LibSQLPhysicalError::ConnectionError(ex))?
        {
            row.get(0).map_err(|ex| LibSQLPhysicalError::ConnectionError(ex))
        } else {
            Ok(0)
        }
    }
    pub async fn read(&mut self, key: &str) -> Result<Option<String>, LibSQLPhysicalError> {
        let table_name = self.libsql_details.table_name.to_string();
        let mut rows = self.get_connection().await?
            .query(
                &format!(
                    "SELECT value_d FROM {table_name} WHERE key_d = ? AND is_deleted_d = 0 ORDER BY version_d DESC LIMIT 1;"
                ),
                libsql::params![key],
            )
            .await
            .map_err(|ex| LibSQLPhysicalError::ConnectionError(ex))?;
        if let Some(row) =
            rows.next().await.map_err(|ex| LibSQLPhysicalError::ConnectionError(ex))?
        {
            Ok(Some(row.get(0).map_err(|ex| LibSQLPhysicalError::ConnectionError(ex))?))
        } else {
            Ok(None)
        }
    }

    pub async fn write(&mut self, key: &str, value: &str) -> Result<(), LibSQLPhysicalError> {
        let table_name = self.libsql_details.table_name.to_string();
        let next_version = self.get_current_version(key).await? + 1;
        let current_epoch_time: i64 =
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        self.get_connection()
            .await?
            .execute(
                &format!("INSERT INTO {table_name} (key_d, value_d, version_d, updated_at_d) VALUES (?, ?, ?, ?);"),
                libsql::params![key, value, next_version, current_epoch_time],
            )
            .await.map_err(|ex| LibSQLPhysicalError::ConnectionError(ex))?;
        Ok(())
    }

    pub async fn delete(&mut self, key: &str) -> Result<(), LibSQLPhysicalError> {
        let table_name = self.libsql_details.table_name.to_string();
        let current_epoch_time: i64 =
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        self.get_connection()
            .await?
            .execute(
                &format!(
                    "UPDATE {table_name} SET is_deleted_d = 1, updated_at_d = ? WHERE key_d = ?;"
                ),
                libsql::params![current_epoch_time, key],
            )
            .await
            .map_err(|ex| LibSQLPhysicalError::ConnectionError(ex))?;
        Ok(())
    }
}
