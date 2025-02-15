mod libsql_store;

use crate::physical::libsql_store::LibSQLPhysical;

#[derive(Clone, Debug)]
pub struct Physical {
    physical_impl: LibSQLPhysical,
}

impl Physical {
    pub fn new(physical: crate::app_settings::Physical) -> Self {
        if physical.physical_type != "libsql" {
            panic!("Only sqlite is supported at this time");
        }
        let physical_impl = LibSQLPhysical::new(physical);
        Physical { physical_impl }
    }

    pub async fn read(&mut self, key: &str) -> Option<String> {
        self.physical_impl.read(key).await
    }

    pub async fn write(&mut self, key: &str, value: &str) {
        self.physical_impl.write(key, value).await
    }

    pub async fn delete(&mut self, key: &str) {
        self.physical_impl.delete(key).await
    }
}
