use std::path::Path;

use rusqlite::{params, Connection};
use uuid::Uuid;

use crate::error::Result;
use crate::model::{Record, SourceObservation};

pub struct ReadModel {
    conn: Connection,
}

impl ReadModel {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        let model = Self { conn };
        model.initialize()?;
        Ok(model)
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        let model = Self { conn };
        model.initialize()?;
        Ok(model)
    }

    fn initialize(&self) -> Result<()> {
        self.conn.execute_batch(
            "
            PRAGMA foreign_keys = ON;
            CREATE TABLE IF NOT EXISTS records (
                id TEXT PRIMARY KEY,
                kind TEXT NOT NULL,
                json TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_records_kind ON records(kind);

            CREATE TABLE IF NOT EXISTS source_observations (
                source_system TEXT NOT NULL,
                object_type TEXT NOT NULL,
                locator TEXT NOT NULL,
                state TEXT,
                status TEXT NOT NULL,
                observed_at TEXT NOT NULL,
                detail TEXT,
                PRIMARY KEY(source_system, object_type, locator)
            );

            CREATE TABLE IF NOT EXISTS metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            ",
        )?;
        Ok(())
    }

    pub fn replace_all(&mut self, records: &[Record], observations: &[SourceObservation]) -> Result<()> {
        let tx = self.conn.transaction()?;
        tx.execute("DELETE FROM records", [])?;
        tx.execute("DELETE FROM source_observations", [])?;
        tx.execute("DELETE FROM metadata", [])?;

        for record in records {
            tx.execute(
                "INSERT INTO records(id, kind, json) VALUES (?1, ?2, ?3)",
                params![record.id().to_string(), record.kind_name(), serde_json::to_string(record)?],
            )?;
        }

        for observation in observations {
            tx.execute(
                "INSERT INTO source_observations(source_system, object_type, locator, state, status, observed_at, detail)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    &observation.source_system,
                    &observation.object_type,
                    &observation.locator,
                    &observation.state,
                    serde_json::to_string(&observation.status)?,
                    &observation.observed_at,
                    &observation.detail,
                ],
            )?;
        }

        tx.execute(
            "INSERT INTO metadata(key, value) VALUES ('record_count', ?1)",
            [records.len().to_string()],
        )?;
        tx.commit()?;
        Ok(())
    }

    pub fn all_records(&self) -> Result<Vec<Record>> {
        let mut statement = self.conn.prepare("SELECT json FROM records ORDER BY kind, id")?;
        let rows = statement.query_map([], |row| row.get::<_, String>(0))?;
        let mut records = Vec::new();
        for row in rows {
            records.push(serde_json::from_str(&row?)?);
        }
        Ok(records)
    }

    pub fn record(&self, id: Uuid) -> Result<Option<Record>> {
        let mut statement = self.conn.prepare("SELECT json FROM records WHERE id = ?1")?;
        let mut rows = statement.query([id.to_string()])?;
        if let Some(row) = rows.next()? {
            let json: String = row.get(0)?;
            Ok(Some(serde_json::from_str(&json)?))
        } else {
            Ok(None)
        }
    }

    pub fn observations(&self) -> Result<Vec<SourceObservation>> {
        let mut statement = self.conn.prepare(
            "SELECT source_system, object_type, locator, state, status, observed_at, detail
             FROM source_observations ORDER BY source_system, object_type, locator",
        )?;
        let rows = statement.query_map([], |row| {
            let status_json: String = row.get(4)?;
            let status = serde_json::from_str(&status_json).map_err(|error| {
                rusqlite::Error::FromSqlConversionFailure(
                    4,
                    rusqlite::types::Type::Text,
                    Box::new(error),
                )
            })?;
            Ok(SourceObservation {
                source_system: row.get(0)?,
                object_type: row.get(1)?,
                locator: row.get(2)?,
                state: row.get(3)?,
                status,
                observed_at: row.get(5)?,
                detail: row.get(6)?,
            })
        })?;

        let mut observations = Vec::new();
        for row in rows {
            observations.push(row?);
        }
        Ok(observations)
    }

    pub fn observation(
        &self,
        source_system: &str,
        object_type: &str,
        locator: &str,
    ) -> Result<Option<SourceObservation>> {
        Ok(self.observations()?.into_iter().find(|observation| {
            observation.source_system == source_system
                && observation.object_type == object_type
                && observation.locator == locator
        }))
    }

    pub fn record_count(&self) -> Result<usize> {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM records", [], |row| row.get(0))?;
        Ok(count as usize)
    }
}
