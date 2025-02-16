mod libsql_store;

use crate::physical::libsql_store::LibSQLPhysical;

#[derive(Clone, Debug)]
pub enum Physical {
    LibSQL(LibSQLPhysical),
}

impl Physical {
    pub fn new(physical: crate::configuration::Physical) -> Self {
        match physical.physical_type.as_str() {
            "libsql" => Physical::LibSQL(LibSQLPhysical::new(physical)),
            _ => panic!("Unsupported storage type"),
        }
    }

    pub async fn read(&mut self, key: &str) -> Option<String> {
        match self {
            Physical::LibSQL(physical_impl) => physical_impl.read(key).await,
        }
    }

    pub async fn write(&mut self, key: &str, value: &str) {
        match self {
            Physical::LibSQL(physical_impl) => physical_impl.write(key, value).await,
        }
    }

    pub async fn delete(&mut self, key: &str) {
        match self {
            Physical::LibSQL(physical_impl) => physical_impl.delete(key).await,
        }
    }
}
