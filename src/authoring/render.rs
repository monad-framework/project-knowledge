use std::fmt::Write;

use crate::authoring::plan::{CapturePlan, PlanOperation};
use crate::model::Record;

pub fn render_plan(plan: &CapturePlan) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "Capture Plan {}", plan.plan_id);
    let _ = writeln!(out, "created: {}", plan.created_at);
    let _ = writeln!(out);

    for operation in &plan.operations {
        render_operation(&mut out, operation);
    }

    let _ = writeln!(out, "Files to create: {}", plan.operations.len());
    let _ = writeln!(out, "Warnings: {}", plan.warnings.len());
    let _ = writeln!(out, "Blockers: {}", plan.blockers.len());
    for warning in &plan.warnings {
        let _ = writeln!(out, "  WARNING [{}] {}", warning.code, warning.message);
    }
    for blocker in &plan.blockers {
        let _ = writeln!(out, "  BLOCKER [{}] {}", blocker.code, blocker.message);
    }
    out
}

fn render_operation(out: &mut String, operation: &PlanOperation) {
    match &operation.record {
        Record::Subject { label, id, .. } => {
            let _ = writeln!(out, "Subject");
            let _ = writeln!(out, "  NEW  {}", label.as_deref().unwrap_or("<unlabeled>"));
            let _ = writeln!(out, "       id: {id}");
        }
        Record::Representation {
            id, role, native, ..
        } => {
            let _ = writeln!(out, "Representation");
            let _ = writeln!(out, "  NEW  {role}");
            let _ = writeln!(out, "       {}", native.locator);
            if let Some(state) = &native.state {
                let _ = writeln!(out, "       observed blob: {state}");
            }
            let _ = writeln!(out, "       id: {id}");
        }
        Record::Claim {
            id, concern, value, ..
        } => {
            let _ = writeln!(out, "Claim");
            let _ = writeln!(out, "  {concern} = {value}");
            let _ = writeln!(out, "  id: {id}");
        }
        Record::Assertion {
            id,
            claim_id,
            representation_id,
            valid_from,
            valid_until,
            ..
        } => {
            let _ = writeln!(out, "Assertion");
            let _ = writeln!(
                out,
                "  claim {claim_id} asserted by representation {representation_id}"
            );
            if let Some(value) = valid_from {
                let _ = writeln!(out, "  valid from: {value}");
            }
            if let Some(value) = valid_until {
                let _ = writeln!(out, "  valid until: {value}");
            }
            let _ = writeln!(out, "  id: {id}");
        }
        Record::Authority {
            id,
            concern,
            representation_id,
            basis,
            valid_from,
            valid_until,
            ..
        } => {
            let _ = writeln!(out, "Authority");
            let _ = writeln!(
                out,
                "  representation {representation_id} governs {concern}"
            );
            let _ = writeln!(out, "  basis: {basis}");
            if let Some(value) = valid_from {
                let _ = writeln!(out, "  valid from: {value}");
            }
            if let Some(value) = valid_until {
                let _ = writeln!(out, "  valid until: {value}");
            }
            let _ = writeln!(out, "  id: {id}");
        }
        Record::Relationship {
            id,
            from,
            relation,
            to,
            origin,
            ..
        } => {
            let _ = writeln!(out, "Relationship");
            let _ = writeln!(
                out,
                "  {:?}:{} --{}--> {:?}:{}",
                from.kind, from.id, relation, to.kind, to.id
            );
            let _ = writeln!(out, "  origin: {origin:?}");
            let _ = writeln!(out, "  id: {id}");
        }
        Record::Activity {
            id,
            activity_type,
            used,
            generated_representation_ids,
            ..
        } => {
            let _ = writeln!(out, "Activity");
            let _ = writeln!(out, "  {activity_type}");
            let _ = writeln!(out, "  used inputs: {}", used.len());
            let _ = writeln!(
                out,
                "  generated representations: {}",
                generated_representation_ids.len()
            );
            let _ = writeln!(out, "  id: {id}");
        }
        Record::Context { id, dimensions, .. } => {
            let _ = writeln!(out, "Context");
            let _ = writeln!(out, "  dimensions: {}", dimensions.len());
            let _ = writeln!(out, "  id: {id}");
        }
        Record::EvidenceEvaluation {
            id,
            claim_id,
            method,
            result,
            inputs,
            ..
        } => {
            let _ = writeln!(out, "Evidence Evaluation");
            let _ = writeln!(out, "  claim: {claim_id}");
            let _ = writeln!(out, "  {method} -> {result:?}");
            let _ = writeln!(out, "  relevant inputs: {}", inputs.len());
            let _ = writeln!(out, "  id: {id}");
        }
    }
    let _ = writeln!(out, "  file: {}", operation.path);
    let _ = writeln!(out);
}
