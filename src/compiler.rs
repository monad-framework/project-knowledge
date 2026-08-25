use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::error::Result;
use crate::git::GitAdapter;
use crate::model::{NativeReference, SourceObservation};
use crate::records::load_records;
use crate::store::ReadModel;

#[derive(Debug, Clone, Serialize)]
pub struct CompileReport {
    pub record_count: usize,
    pub observation_count: usize,
    pub enriched: bool,
    pub database: Option<PathBuf>,
}

pub fn default_db_path(root: &Path) -> PathBuf {
    root.join(".pk").join("cache").join("read-model.sqlite3")
}

pub fn compile(root: &Path) -> Result<CompileReport> {
    let db_path = default_db_path(root);
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let records = load_records(root)?;
    let observations = collect_observations(root, &records);
    let mut model = ReadModel::open(&db_path)?;
    model.replace_all(&records, &observations)?;
    Ok(CompileReport {
        record_count: records.len(),
        observation_count: observations.len(),
        enriched: !records.is_empty(),
        database: Some(db_path),
    })
}

pub fn rebuild(root: &Path) -> Result<CompileReport> {
    let db_path = default_db_path(root);
    if db_path.exists() {
        fs::remove_file(&db_path)?;
    }
    compile(root)
}

pub fn compile_in_memory(root: &Path) -> Result<(ReadModel, CompileReport)> {
    let records = load_records(root)?;
    let observations = collect_observations(root, &records);
    let mut model = ReadModel::in_memory()?;
    model.replace_all(&records, &observations)?;
    let report = CompileReport {
        record_count: records.len(),
        observation_count: observations.len(),
        enriched: !records.is_empty(),
        database: None,
    };
    Ok((model, report))
}

fn collect_observations(root: &Path, records: &[crate::model::Record]) -> Vec<SourceObservation> {
    let adapter = GitAdapter;
    let mut refs: BTreeMap<(String, String, String), NativeReference> = BTreeMap::new();
    refs.insert(
        ("git".to_string(), "repository".to_string(), ".".to_string()),
        NativeReference {
            source_system: "git".to_string(),
            object_type: "repository".to_string(),
            locator: ".".to_string(),
            state: None,
        },
    );

    for record in records {
        for native in record.native_references() {
            refs.entry(native.key()).or_insert_with(|| native.clone());
        }
    }

    let mut seen = BTreeSet::new();
    refs.into_values()
        .filter_map(|native| {
            if seen.insert(native.key()) {
                Some(adapter.observe_reference(root, &native))
            } else {
                None
            }
        })
        .collect()
}
