#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let _ = orbitdeck::score_telemetry_bytes(data);
});
