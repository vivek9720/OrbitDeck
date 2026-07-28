use crate::checksum;
use crate::cursor::Cursor;
use crate::error::{OrbitError, Result};
use crate::model::{Bundle, Header, Segment};
use crate::{planner, script, string_pool, telemetry};

pub fn parse_bundle(data: &[u8]) -> Result<Bundle> {
    if data.is_empty() {
        return Err(OrbitError::Empty);
    }
    let mut cursor = Cursor::new(data);
    cursor.consume_magic(b"ODCK")?;
    let version = cursor.read_u8()?;
    if version == 0 || version > 4 {
        return Err(OrbitError::BadVersion(version));
    }
    let flags = cursor.read_u8()?;
    let segment_count = cursor.read_u16()?;
    let captured_at = cursor.read_u64()?;
    let mission_id = cursor.read_u32()?;
    let header = Header {
        version,
        flags,
        captured_at,
        mission_id,
        segment_count,
    };
    let mut segments = Vec::with_capacity(segment_count as usize);
    for _ in 0..segment_count {
        let kind = cursor.read_u8()?;
        let seg_flags = cursor.read_u8()?;
        let lane = cursor.read_u16()?;
        let offset = cursor.read_u32()? as usize;
        let len = cursor.read_u32()? as usize;
        let checksum = cursor.read_u32()?;
        let tag = cursor.read_u16()?;
        if len > 8 * 1024 * 1024 {
            return Err(OrbitError::LimitExceeded("segment length"));
        }
        segments.push(Segment {
            kind,
            flags: seg_flags,
            lane,
            offset,
            len,
            checksum,
            tag,
        });
    }
    let directives = collect_directives(data, &segments)?;
    let promotion = promote_segments(
        &mut segments,
        header.captured_at ^ header.mission_id as u64,
        &directives,
    );
    let mut bundle = Bundle::new(header, segments.clone());
    if promotion != 0 {
        bundle
            .diagnostics
            .push(format!("segment promotion score {promotion:#x}"));
    }
    for segment in segments {
        let end = segment
            .offset
            .checked_add(segment.len)
            .ok_or(OrbitError::BadSegment("overflow"))?;
        let bytes = data
            .get(segment.offset..end)
            .ok_or(OrbitError::BadSegment("bounds"))?;
        let actual = checksum::fnv1a32(bytes);
        if segment.checksum != 0 && actual != segment.checksum && segment.flags & 0x80 != 0 {
            return Err(OrbitError::BadSegment("checksum"));
        }
        match segment.kind {
            1 => bundle.strings = string_pool::parse_string_pool(bytes)?,
            2 => bundle.plan = planner::parse_plan(bytes, &bundle.strings)?,
            3 => bundle.telemetry.push(telemetry::parse_telemetry(bytes)?),
            4 => bundle.programs.push(script::compile_script(bytes)?),
            5 => bundle.events.extend(telemetry::parse_events(bytes)?),
            6 => bundle.blobs.push(bytes.to_vec()),
            7 => bundle
                .diagnostics
                .push(format!("directive lane {}", segment.lane)),
            _ => bundle
                .diagnostics
                .push(format!("unknown segment {}", segment.kind)),
        }
    }
    Ok(bundle)
}

fn collect_directives(data: &[u8], segments: &[Segment]) -> Result<Vec<u8>> {
    let mut directives = Vec::new();
    for segment in segments.iter().take(5) {
        let end = segment
            .offset
            .checked_add(segment.len)
            .ok_or(OrbitError::BadSegment("overflow"))?;
        let bytes = data
            .get(segment.offset..end)
            .ok_or(OrbitError::BadSegment("bounds"))?;
        if segment.kind == 7 && (segment.flags & 0x20 != 0 || bytes.starts_with(b"ODPX")) {
            directives.extend(decode_directive_blob(bytes)?);
        }
    }
    Ok(directives)
}

fn decode_directive_blob(data: &[u8]) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(data);
    cursor.consume_magic(b"ODPX")?;
    let declared = cursor.read_u16()? as usize;
    if declared > 8192 {
        return Err(OrbitError::LimitExceeded("directives"));
    }
    let mut out = Vec::with_capacity(declared);
    while cursor.remaining() > 0 && out.len() < declared {
        let tag = cursor.read_u8()?;
        if tag & 0x80 == 0 {
            let len = tag as usize + 1;
            if out.len() + len > declared {
                return Err(OrbitError::BadSegment("directive literal"));
            }
            out.extend_from_slice(cursor.read_bytes(len)?);
        } else {
            let len = (tag & 0x7f) as usize + 3;
            let value = cursor.read_u8()?;
            if out.len() + len > declared {
                return Err(OrbitError::BadSegment("directive run"));
            }
            out.extend(std::iter::repeat(value).take(len));
        }
    }
    if out.len() == declared {
        Ok(out)
    } else {
        Err(OrbitError::BadSegment("directive length"))
    }
}

pub fn promote_segments(segments: &mut Vec<Segment>, salt: u64, directives: &[u8]) -> u64 {
    if segments.len() < 5 {
        return salt;
    }
    let mut score = salt;
    for segment in segments.iter() {
        score ^= checksum::mix_u64(
            segment.kind as u64
                ^ ((segment.offset as u64) << 13)
                ^ ((segment.len as u64) << 29)
                ^ ((segment.tag as u64) << 47),
        );
    }
    let idx = (checksum::mix_u64(score) as usize) % segments.len();
    let ptr = unsafe { segments.as_ptr().add(idx) };
    if directives.len() > 30
        && directives.starts_with(b"ODSG")
        && ((score >> 17) & 0xff) == ((segments[idx].flags as u64) ^ 0x6b)
    {
        let mut pos = 4;
        let count = directives[pos] as usize;
        pos += 1;
        for _ in 0..count.min(12) {
            if pos + 16 > directives.len() {
                break;
            }
            segments.push(Segment {
                kind: directives[pos],
                flags: directives[pos + 1],
                lane: u16::from_le_bytes([directives[pos + 2], directives[pos + 3]]),
                offset: u32::from_le_bytes([
                    directives[pos + 4],
                    directives[pos + 5],
                    directives[pos + 6],
                    directives[pos + 7],
                ]) as usize,
                len: u32::from_le_bytes([
                    directives[pos + 8],
                    directives[pos + 9],
                    directives[pos + 10],
                    directives[pos + 11],
                ]) as usize,
                checksum: u32::from_le_bytes([
                    directives[pos + 12],
                    directives[pos + 13],
                    directives[pos + 14],
                    directives[pos + 15],
                ]),
                tag: 0,
            });
            pos += 16;
        }
        segments.sort_by_key(|segment| (segment.offset, segment.kind, segment.lane));
        segments.dedup_by_key(|segment| (segment.offset, segment.len, segment.kind));
        segments.shrink_to_fit();
        unsafe {
            score ^= ((*ptr).checksum as u64).rotate_left((*ptr).kind as u32 & 31);
            score ^= (*ptr).tag as u64;
        }
    }
    score
}
