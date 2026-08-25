use std::path::{Path, PathBuf};

use crate::authoring::intent::ExistingSelector;
use crate::error::{Error, Result};
use crate::model::{EntityKind, Record};
use crate::records::load_records;

#[derive(Debug, Clone)]
pub struct CatalogMatch {
    pub record: Record,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct RecordCatalog {
    records: Vec<Record>,
}

impl RecordCatalog {
    pub fn load(root: &Path) -> Result<Self> {
        Ok(Self {
            records: load_records(root)?,
        })
    }

    pub fn records(&self) -> &[Record] {
        &self.records
    }

    pub fn resolve(&self, selector: &ExistingSelector) -> Result<CatalogMatch> {
        let mut matches: Vec<Record> = self
            .records
            .iter()
            .filter(|record| record_kind(record) == selector.kind)
            .filter(|record| {
                selector.id.is_none_or(|id| record.id() == id)
                    && selector
                        .label
                        .as_deref()
                        .is_none_or(|label| record_label(record) == Some(label))
                    && selector
                        .locator
                        .as_deref()
                        .is_none_or(|locator| record_locator(record) == Some(locator))
            })
            .cloned()
            .collect();

        match matches.len() {
            0 => Err(Error::AuthoringInput(format!(
                "existing {:?} selector matched no records",
                selector.kind
            ))),
            1 => {
                let record = matches.remove(0);
                Ok(CatalogMatch {
                    path: record_relative_path(&record),
                    record,
                })
            }
            count => Err(Error::AuthoringInput(format!(
                "existing {:?} selector is ambiguous: {count} matches",
                selector.kind
            ))),
        }
    }
}

pub fn record_relative_path(record: &Record) -> PathBuf {
    PathBuf::from(".pk")
        .join("records")
        .join(record.kind_name())
        .join(format!("{}.json", record.id()))
}

pub fn record_kind(record: &Record) -> EntityKind {
    match record {
        Record::Subject { .. } => EntityKind::Subject,
        Record::Representation { .. } => EntityKind::Representation,
        Record::Claim { .. } => EntityKind::Claim,
        Record::Assertion { .. } => EntityKind::Assertion,
        Record::Authority { .. } => EntityKind::Authority,
        Record::Relationship { .. } => EntityKind::Relationship,
        Record::Activity { .. } => EntityKind::Activity,
        Record::Context { .. } => EntityKind::Context,
        Record::EvidenceEvaluation { .. } => EntityKind::EvidenceEvaluation,
    }
}

fn record_label(record: &Record) -> Option<&str> {
    match record {
        Record::Subject { label, .. } => label.as_deref(),
        _ => None,
    }
}

fn record_locator(record: &Record) -> Option<&str> {
    match record {
        Record::Representation { native, .. } => Some(native.locator.as_str()),
        _ => None,
    }
}
