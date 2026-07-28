#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let _ = orbitdeck::decode_and_schedule_bundle(data);
});
