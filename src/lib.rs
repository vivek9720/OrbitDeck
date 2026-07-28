//! OrbitDeck decodes offline satellite ground-station scheduling bundles.

pub mod analysis;
pub mod arena;
pub mod checksum;
pub mod codec;
pub mod cursor;
pub mod decoder;
pub mod error;
pub mod model;
pub mod orbit_catalog;
pub mod planner;
pub mod policy_table;
pub mod script;
pub mod string_pool;
pub mod telemetry;

pub use analysis::Analyzer;
pub use error::{OrbitError, Result};
pub use model::{Bundle, Header, Plan, ScheduleFinding, ScheduleReport, TelemetrySet};

pub fn parse_bundle(data: &[u8]) -> Result<Bundle> {
    decoder::parse_bundle(data)
}

pub fn decode_and_schedule_bundle(data: &[u8]) -> Result<ScheduleReport> {
    let bundle = parse_bundle(data)?;
    Ok(Analyzer::new().analyze(&bundle))
}

pub fn decode_stream(data: &[u8]) -> Result<Bundle> {
    codec::decode_stream(data)
}

pub fn decode_stream_and_schedule(data: &[u8]) -> Result<ScheduleReport> {
    codec::decode_stream_and_schedule(data)
}

pub fn score_plan_bytes(data: &[u8]) -> Result<u64> {
    planner::parse_and_score_plan(data)
}

pub fn score_telemetry_bytes(data: &[u8]) -> Result<u64> {
    telemetry::parse_and_score_telemetry(data)
}

pub fn replay_eventlog_bytes(data: &[u8]) -> Result<u64> {
    let mut events = telemetry::parse_events(data)?;
    Ok(telemetry::replay_events(&mut events))
}

pub fn run_script_bytes(data: &[u8]) -> Result<script::ExecutionReport> {
    script::compile_and_run(data)
}
