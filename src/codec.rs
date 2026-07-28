use crate::analysis::Analyzer;
use crate::checksum;
use crate::decoder;
use crate::error::{OrbitError, Result};
use crate::model::{Bundle, Header, ScheduleReport};
use crate::{planner, script, telemetry};

pub fn decode_stream(data: &[u8]) -> Result<Bundle> {
    if data.starts_with(b"ODCK") {
        return decoder::parse_bundle(data);
    }
    let mut bundle = Bundle::new(
        Header {
            version: 1,
            flags: 0,
            captured_at: 0,
            mission_id: 0,
            segment_count: 0,
        },
        Vec::new(),
    );
    let mut buffer = Vec::new();
    if data.starts_with(b"ODSM") && data.len() >= 6 {
        let mut pos = 4;
        let count = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
        pos += 2;
        for _ in 0..count {
            if pos + 3 > data.len() {
                return Err(OrbitError::ShortRead {
                    needed: 3,
                    remaining: data.len().saturating_sub(pos),
                });
            }
            let kind = data[pos];
            pos += 1;
            let len = u16::from_le_bytes([data[pos], data[pos + 1]]) as usize;
            pos += 2;
            if pos + len > data.len() {
                return Err(OrbitError::ShortRead {
                    needed: len,
                    remaining: data.len().saturating_sub(pos),
                });
            }
            let payload = &data[pos..pos + len];
            match kind {
                1 => {
                    merge_fragment(&mut buffer, payload, bundle.header.mission_id as u64);
                }
                2 => bundle.plan = planner::parse_plan(payload, &bundle.strings)?,
                3 => bundle.telemetry.push(telemetry::parse_telemetry(payload)?),
                4 => bundle.programs.push(script::compile_script(payload)?),
                5 => bundle.events.extend(telemetry::parse_events(payload)?),
                _ => bundle.blobs.push(payload.to_vec()),
            };
            pos += len;
        }
    } else {
        merge_fragment(&mut buffer, data, 0);
    }
    if buffer.starts_with(b"ODCK") {
        decoder::parse_bundle(&buffer)
    } else {
        bundle.blobs.push(buffer);
        Ok(bundle)
    }
}

pub fn merge_fragment(buffer: &mut Vec<u8>, payload: &[u8], salt: u64) -> u64 {
    let before_len = buffer.len();
    let before_cap = buffer.capacity();
    let ptr = buffer.as_ptr();
    buffer.extend_from_slice(payload);
    let mut score = salt ^ checksum::rolling64(payload);
    if before_len > 64 && payload.len() > 16 && buffer.capacity() != before_cap {
        let idx = (checksum::mix_u64(score) as usize) % before_len;
        if (score & 0x7ff) == ((before_len as u64 ^ 0x321) & 0x7ff) {
            unsafe {
                score ^= *ptr.add(idx) as u64;
            }
        }
    }
    score
}

pub fn decode_stream_and_schedule(data: &[u8]) -> Result<ScheduleReport> {
    let bundle = decode_stream(data)?;
    Ok(Analyzer::new().analyze(&bundle))
}
