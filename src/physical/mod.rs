mod libsql_store;

use crate::physical::libsql_store::LibSQLPhysical;

#[derive(Clone, Debug)]
pub enum Physical {
    LibSQL(LibSQLPhysical),
}

pub enum PhysicalError {
    LibSQL(libsql_store::LibSQLPhysicalError),
}

impl Physical {
    pub fn new(physical: crate::configuration::Physical) -> Self {
        match physical.physical_type.as_str() {
            "libsql" => Physical::LibSQL(LibSQLPhysical::new(physical)),
            _ => panic!("Unsupported storage type"),
        }
    }

    pub async fn read(&mut self, key: &str) -> Result<Option<String>, PhysicalError> {
        match self {
            Physical::LibSQL(physical_impl) => {
                physical_impl.read(key).await.map_err(|e| PhysicalError::LibSQL(e))
            }
        }
    }

    pub async fn write(&mut self, key: &str, value: &str) -> Result<(), PhysicalError> {
        match self {
            Physical::LibSQL(physical_impl) => {
                physical_impl.write(key, value).await.map_err(|e| PhysicalError::LibSQL(e))
            }
        }
    }

    pub async fn delete(&mut self, key: &str) -> Result<(), PhysicalError> {
        match self {
            Physical::LibSQL(physical_impl) => {
                physical_impl.delete(key).await.map_err(|e| PhysicalError::LibSQL(e))
            }
        }
    }
}
