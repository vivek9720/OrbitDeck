#![no_main]

libfuzzer_sys::fuzz_target!(|data: &[u8]| {
    let _ = orbitdeck::run_script_bytes(data);
});
