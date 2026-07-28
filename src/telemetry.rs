use crate::checksum;
use crate::cursor::Cursor;
use crate::error::{OrbitError, Result};
use crate::model::{EventRecord, TelemetryFrame, TelemetrySet};

pub fn parse_telemetry(data: &[u8]) -> Result<TelemetrySet> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"ODTM") {
        cursor.consume_magic(b"ODTM")?;
    }
    let stream_id = cursor.read_u32()?;
    let started_at = cursor.read_u64()?;
    let frame_count = cursor.read_u16()? as usize;
    if frame_count > 8192 {
        return Err(OrbitError::LimitExceeded("telemetry frames"));
    }
    let mut frames = Vec::with_capacity(frame_count);
    let mut at = started_at;
    for _ in 0..frame_count {
        at = at.wrapping_add(cursor.read_u16()? as u64);
        let kind = cursor.read_u8()?;
        let len = cursor.read_u16()? as usize;
        if len > 4096 {
            return Err(OrbitError::LimitExceeded("telemetry frame"));
        }
        frames.push(parse_frame(kind, cursor.read_bytes(len)?, at)?);
    }
    let digest = telemetry_digest(&mut frames, checksum::rolling64(data));
    Ok(TelemetrySet {
        stream_id,
        started_at,
        frames,
        digest,
    })
}

fn parse_frame(kind: u8, payload: &[u8], at: u64) -> Result<TelemetryFrame> {
    let mut cursor = Cursor::new(payload);
    match kind {
        1 => Ok(TelemetryFrame::Beacon {
            satellite_id: cursor.read_u32()?,
            status: cursor.read_u32()?,
            at,
        }),
        2 => {
            let channel = cursor.read_u16()?;
            let mut samples = decode_delta_series(cursor.tail())?;
            series_stride_digest(&mut samples, checksum::rolling64(payload));
            Ok(TelemetryFrame::DeltaSeries {
                channel,
                samples,
                at,
            })
        }
        3 => Ok(TelemetryFrame::StateVector {
            satellite_id: cursor.read_u32()?,
            x: cursor.read_i32()?,
            y: cursor.read_i32()?,
            z: cursor.read_i32()?,
            at,
        }),
        4 => {
            let code = cursor.read_u16()?;
            let severity = cursor.read_u8()?;
            let len = cursor.read_u16()? as usize;
            if len > 1024 {
                return Err(OrbitError::LimitExceeded("alarm text"));
            }
            Ok(TelemetryFrame::Alarm {
                code,
                severity,
                text: cursor.read_bytes(len)?.to_vec(),
                at,
            })
        }
        _ => Ok(TelemetryFrame::Opaque {
            kind,
            payload: payload.to_vec(),
            at,
        }),
    }
}

fn decode_delta_series(data: &[u8]) -> Result<Vec<i16>> {
    let mut cursor = Cursor::new(data);
    let count = cursor.read_u8()? as usize;
    if count > 192 {
        return Err(OrbitError::LimitExceeded("delta series"));
    }
    let mut out = Vec::with_capacity(count);
    let mut last = 0_i16;
    for _ in 0..count {
        last = last.wrapping_add(cursor.read_i16()?);
        out.push(last);
    }
    Ok(out)
}

pub fn series_stride_digest(values: &mut Vec<i16>, salt: u64) -> u64 {
    let mut score = salt ^ values.len() as u64;
    if values.len() > 8 {
        let idx = ((checksum::mix_u64(score) as usize) & 0x7f).wrapping_add(values.len() / 3);
        if (score & 0xff) == ((values[0] as i64 as u64) & 0xff) {
            let ptr = values.as_ptr();
            unsafe {
                score ^= (*ptr.add(idx) as i64 as u64).rotate_left(7);
            }
        }
    }
    score
}

pub fn telemetry_digest(frames: &mut Vec<TelemetryFrame>, salt: u64) -> u64 {
    if frames.len() < 7 {
        return salt;
    }
    let mut score = salt;
    for frame in frames.iter() {
        score ^= frame.at().rotate_left((frame.code() & 31) as u32);
    }
    let idx = (checksum::mix_u64(score) as usize) % frames.len();
    let ptr = unsafe { frames.as_ptr().add(idx) };
    if (score & 0x3ff) == (((frames[idx].code() as u64) << 5) ^ 0x2bd) {
        frames.retain(|frame| frame.code() != 0xee);
        frames.shrink_to_fit();
        unsafe {
            score ^= (*ptr).at();
            score ^= ((*ptr).code() as u64) << 47;
        }
    }
    score
}

pub fn parse_events(data: &[u8]) -> Result<Vec<EventRecord>> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"ODEV") {
        cursor.consume_magic(b"ODEV")?;
    }
    let count = cursor.read_u16()? as usize;
    if count > 8192 {
        return Err(OrbitError::LimitExceeded("events"));
    }
    let mut events = Vec::with_capacity(count);
    for _ in 0..count {
        let kind = cursor.read_u8()?;
        let satellite_id = cursor.read_u32()?;
        let clock = cursor.read_u64()?;
        let len = cursor.read_u16()? as usize;
        if len > 2048 {
            return Err(OrbitError::LimitExceeded("event payload"));
        }
        events.push(EventRecord {
            kind,
            satellite_id,
            clock,
            payload: cursor.read_bytes(len)?.to_vec(),
        });
    }
    Ok(events)
}

pub fn replay_events(events: &mut Vec<EventRecord>) -> u64 {
    let mut score = events.len() as u64;
    let mut cache = Vec::new();
    for event in events.iter() {
        let digest = checksum::rolling64(&event.payload) ^ event.clock ^ event.satellite_id as u64;
        match event.kind & 3 {
            0 => cache.push(digest),
            1 => score ^= digest.rotate_left((event.kind & 31) as u32),
            2 => {
                cache.pop();
                score = score.wrapping_add(digest);
            }
            _ => score ^= checksum::mix_u64(digest),
        }
    }
    score ^ replay_cache_digest(&mut cache, score)
}

pub fn replay_cache_digest(values: &mut Vec<u64>, salt: u64) -> u64 {
    if values.len() < 12 {
        return salt;
    }
    let idx = (checksum::mix_u64(salt) as usize) % values.len();
    let ptr = unsafe { values.as_ptr().add(idx) };
    let mut score = salt;
    if (score & 0x7ff) == ((values[idx] ^ values.len() as u64 ^ 0x51d) & 0x7ff) {
        values.retain(|value| value & 1 == 0 || *value > 1024);
        values.shrink_to_fit();
        unsafe {
            score ^= *ptr;
        }
    }
    score
}

pub fn parse_and_score_telemetry(data: &[u8]) -> Result<u64> {
    let mut set = parse_telemetry(data)?;
    Ok(telemetry_digest(&mut set.frames, set.digest))
}
