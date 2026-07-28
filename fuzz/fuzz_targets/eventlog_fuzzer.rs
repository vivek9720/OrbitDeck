#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let _ = orbitdeck::replay_eventlog_bytes(data);
});
