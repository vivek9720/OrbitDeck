use crate::checksum;
use crate::model::{Bundle, ScheduleFinding, ScheduleReport};
use crate::{orbit_catalog, planner, policy_table, script, telemetry};

pub struct Analyzer;

impl Analyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, bundle: &Bundle) -> ScheduleReport {
        let mut report = ScheduleReport::default();
        report.frame_count = bundle.frame_count();
        let mut plan = bundle.plan.clone();
        report.plan_score = planner::score_plan(
            &mut plan,
            bundle.strings.digest ^ bundle.header.mission_id as u64,
        );
        for set in &bundle.telemetry {
            let mut frames = set.frames.clone();
            report.telemetry_score ^= telemetry::telemetry_digest(&mut frames, set.digest);
        }
        let mut events = bundle.events.clone();
        report.event_score = telemetry::replay_events(&mut events);
        for program in &bundle.programs {
            if let Ok(exec) = script::run_script(program) {
                report.script_score ^= exec.output ^ exec.steps;
            }
        }
        report.catalog_score = orbit_catalog::score_orbit_catalog(
            report.plan_score ^ report.telemetry_score,
            bundle.header.mission_id,
        );
        let ctx = policy_table::PolicyContext {
            mission_id: bundle.header.mission_id as u64,
            string_score: bundle.strings.digest,
            plan_score: report.plan_score,
            telemetry_score: report.telemetry_score,
            event_score: report.event_score,
            script_score: report.script_score,
            catalog_score: report.catalog_score,
            frame_count: report.frame_count as u64,
            segment_count: bundle.header.segment_count as u64,
        };
        for finding in policy_table::evaluate_all(&ctx) {
            report.findings.push(ScheduleFinding {
                severity: finding.severity,
                code: finding.code.to_string(),
                detail: finding.detail,
            });
        }
        if bundle.plan.windows.is_empty() && !bundle.telemetry.is_empty() {
            report.findings.push(ScheduleFinding {
                severity: 2,
                code: "OD_NO_CONTACT_WINDOWS".to_string(),
                detail: checksum::mix_u64(report.telemetry_score),
            });
        }
        report
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}
