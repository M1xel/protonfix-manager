use anyhow::Result;
use redb::{Database, ReadableDatabase, TableDefinition};
use serde::{Deserialize, Serialize};

const TABLE: TableDefinition<&str, &str> = TableDefinition::new("data");

pub struct AppDb {
    db: Database,
}

impl AppDb {
    pub fn new(path: &str) -> Result<Self> {
        Ok(Self {
            db: Database::create(path)?,
        })
    }

    pub fn save<T: Serialize>(&self, key: &str, value: &T) -> Result<()> {
        let json = serde_json::to_string(value)?;
        let txn = self.db.begin_write()?;
        txn.open_table(TABLE)?.insert(key, json.as_str())?;
        txn.commit()?;
        Ok(())
    }

    pub fn load<T: for<'a> Deserialize<'a>>(&self, key: &str) -> Result<Option<T>> {
        let txn = self.db.begin_read()?;
        let table = txn.open_table(TABLE)?;
        Ok(table
            .get(key)?
            .map(|v| serde_json::from_str(v.value()).unwrap()))
    }
}
