#[test]
fn bundle_seed_decodes_and_schedules() {
    let data = include_bytes!("../fuzz/corpus/bundle_fuzzer/seed_bundle.odck");
    let report = orbitdeck::decode_and_schedule_bundle(data).expect("bundle seed should parse");
    assert!(report.frame_count > 0);
}

#[test]
fn entrypoint_seeds_reach_library_code() {
    let stream = include_bytes!("../fuzz/corpus/stream_fuzzer/seed_stream.odsm");
    let plan = include_bytes!("../fuzz/corpus/plan_fuzzer/seed_plan.odpl");
    let telemetry = include_bytes!("../fuzz/corpus/telemetry_fuzzer/seed_telemetry.odtm");
    let script = include_bytes!("../fuzz/corpus/script_fuzzer/seed_script.odsc");
    let events = include_bytes!("../fuzz/corpus/eventlog_fuzzer/seed_eventlog.odev");

    let stream_report = orbitdeck::decode_stream_and_schedule(stream).expect("stream seed");
    let plan_score = orbitdeck::score_plan_bytes(plan).expect("plan seed");
    let telemetry_score = orbitdeck::score_telemetry_bytes(telemetry).expect("telemetry seed");
    let script_report = orbitdeck::run_script_bytes(script).expect("script seed");
    let event_score = orbitdeck::replay_eventlog_bytes(events).expect("event seed");

    assert!(stream_report.frame_count > 0);
    assert_ne!(plan_score, 0);
    assert_ne!(telemetry_score, 0);
    assert!(script_report.steps > 0);
    assert_ne!(event_score, 0);
}
