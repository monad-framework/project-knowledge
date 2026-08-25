use std::collections::{BTreeSet, HashMap};

use jiff::Timestamp;
use serde::Serialize;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::model::{EvidenceResult, ObservationStatus, Record, window_contains};
use crate::store::ReadModel;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionOutcome {
    Resolved,
    Compatible,
    Conflict,
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub struct Resolution {
    pub subject_id: Uuid,
    pub concern: String,
    pub outcome: ResolutionOutcome,
    pub claim_ids: Vec<Uuid>,
    pub authority_assignment_ids: Vec<Uuid>,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Freshness {
    Current,
    Stale,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceState {
    Current,
    Stale,
    Failed,
    Inconclusive,
    Unknown,
}

impl ReadModel {
    pub fn resolve_current(
        &self,
        subject_id: Uuid,
        concern: &str,
        at: Option<&str>,
        context_id: Option<Uuid>,
    ) -> Result<Resolution> {
        let at = match at {
            Some(value) => value.parse::<Timestamp>()?,
            None => Timestamp::now(),
        };
        let records = self.all_records()?;

        let claims: HashMap<Uuid, &Record> = records
            .iter()
            .filter_map(|record| match record {
                Record::Claim {
                    id,
                    subject_id: record_subject,
                    concern: record_concern,
                    ..
                } if *record_subject == subject_id && record_concern == concern => Some((*id, record)),
                _ => None,
            })
            .collect();

        let authority: Vec<(Uuid, Uuid)> = records
            .iter()
            .filter_map(|record| match record {
                Record::Authority {
                    id,
                    subject_id: record_subject,
                    concern: record_concern,
                    representation_id,
                    valid_from,
                    valid_until,
                    context_id: authority_context,
                    ..
                } if *record_subject == subject_id
                    && record_concern == concern
                    && context_matches(*authority_context, context_id)
                    && window_contains(valid_from.as_deref(), valid_until.as_deref(), &at).ok()? =>
                {
                    Some((*id, *representation_id))
                }
                _ => None,
            })
            .collect();

        if authority.is_empty() {
            return Ok(Resolution {
                subject_id,
                concern: concern.to_string(),
                outcome: ResolutionOutcome::Unknown,
                claim_ids: Vec::new(),
                authority_assignment_ids: Vec::new(),
                explanation: "no applicable authority assignment exists".to_string(),
            });
        }

        let authority_representations: BTreeSet<Uuid> =
            authority.iter().map(|(_, representation)| *representation).collect();
        let mut active_claims = BTreeSet::new();
        for record in &records {
            if let Record::Assertion {
                claim_id,
                representation_id,
                valid_from,
                valid_until,
                context_id: assertion_context,
                ..
            } = record
                && claims.contains_key(claim_id)
                && authority_representations.contains(representation_id)
                && context_matches(*assertion_context, context_id)
                && window_contains(valid_from.as_deref(), valid_until.as_deref(), &at)?
            {
                active_claims.insert(*claim_id);
            }
        }

        let authority_ids = authority.iter().map(|(id, _)| *id).collect::<Vec<_>>();
        let claim_ids = active_claims.iter().copied().collect::<Vec<_>>();

        let (outcome, explanation) = match active_claims.len() {
            0 => (
                ResolutionOutcome::Unknown,
                "authority exists but no applicable authoritative assertion was found".to_string(),
            ),
            1 if authority_representations.len() > 1 => (
                ResolutionOutcome::Compatible,
                "all applicable authoritative representations assert the same Claim".to_string(),
            ),
            1 => (
                ResolutionOutcome::Resolved,
                "one applicable authoritative Claim was resolved".to_string(),
            ),
            _ => (
                ResolutionOutcome::Conflict,
                "applicable authoritative representations assert incompatible Claim identities"
                    .to_string(),
            ),
        };

        Ok(Resolution {
            subject_id,
            concern: concern.to_string(),
            outcome,
            claim_ids,
            authority_assignment_ids: authority_ids,
            explanation,
        })
    }

    pub fn representation_freshness(&self, representation_id: Uuid) -> Result<Freshness> {
        let records = self.all_records()?;
        let mut activities = records
            .iter()
            .filter_map(|record| match record {
                Record::Activity {
                    recorded_at,
                    used,
                    generated_representation_ids,
                    ..
                } if generated_representation_ids.contains(&representation_id) => {
                    Some((recorded_at.parse::<Timestamp>().ok()?, used))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        activities.sort_by(|(left, _), (right, _)| left.cmp(right));
        let Some((_, used)) = activities.last() else {
            return Ok(Freshness::Unknown);
        };
        compare_inputs(self, used)
    }

    pub fn evidence_state(&self, evaluation_id: Uuid) -> Result<EvidenceState> {
        let record = self
            .record(evaluation_id)?
            .ok_or_else(|| Error::NotFound(evaluation_id.to_string()))?;
        let Record::EvidenceEvaluation { result, inputs, .. } = record else {
            return Err(Error::NotFound(format!(
                "{evaluation_id} is not an evidence evaluation"
            )));
        };

        match result {
            EvidenceResult::Fail => Ok(EvidenceState::Failed),
            EvidenceResult::Inconclusive => Ok(EvidenceState::Inconclusive),
            EvidenceResult::Pass => match compare_inputs(self, &inputs)? {
                Freshness::Current => Ok(EvidenceState::Current),
                Freshness::Stale => Ok(EvidenceState::Stale),
                Freshness::Unknown => Ok(EvidenceState::Unknown),
            },
        }
    }

    pub fn evidence_for_claim(&self, claim_id: Uuid) -> Result<Vec<Uuid>> {
        Ok(self
            .all_records()?
            .into_iter()
            .filter_map(|record| match record {
                Record::EvidenceEvaluation {
                    id,
                    claim_id: id_claim,
                    ..
                } if id_claim == claim_id => Some(id),
                _ => None,
            })
            .collect())
    }

    pub fn representations_for_subject(&self, subject_id: Uuid) -> Result<Vec<Uuid>> {
        Ok(self
            .all_records()?
            .into_iter()
            .filter_map(|record| match record {
                Record::Representation {
                    id,
                    subject_id: Some(id_subject),
                    ..
                } if id_subject == subject_id => Some(id),
                _ => None,
            })
            .collect())
    }
}

fn context_matches(record_context: Option<Uuid>, requested_context: Option<Uuid>) -> bool {
    record_context.is_none() || record_context == requested_context
}

fn compare_inputs(
    model: &ReadModel,
    inputs: &[crate::model::NativeReference],
) -> Result<Freshness> {
    if inputs.is_empty() {
        return Ok(Freshness::Unknown);
    }

    let mut saw_unknown = false;
    for input in inputs {
        let Some(expected) = input.state.as_deref() else {
            saw_unknown = true;
            continue;
        };
        let Some(observation) = model.observation(
            &input.source_system,
            &input.object_type,
            &input.locator,
        )? else {
            saw_unknown = true;
            continue;
        };

        match observation.status {
            ObservationStatus::Available => {
                if observation.state.as_deref() != Some(expected) {
                    return Ok(Freshness::Stale);
                }
            }
            ObservationStatus::Missing => return Ok(Freshness::Stale),
            ObservationStatus::NotRepository
            | ObservationStatus::Unavailable
            | ObservationStatus::Unsupported => saw_unknown = true,
        }
    }

    if saw_unknown {
        Ok(Freshness::Unknown)
    } else {
        Ok(Freshness::Current)
    }
}
