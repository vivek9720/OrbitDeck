use crate::arena::{self, HandleArena};
use crate::checksum;
use crate::cursor::Cursor;
use crate::error::{OrbitError, Result};
use crate::model::{Antenna, ContactWindow, GroundStation, OrbitArc, Plan, StringPool};
use crate::orbit_catalog;

pub fn parse_plan(data: &[u8], _pool: &StringPool) -> Result<Plan> {
    let mut cursor = Cursor::new(data);
    if cursor.starts_with(b"ODPL") {
        cursor.consume_magic(b"ODPL")?;
    }
    let station_count = cursor.read_u16()? as usize;
    let antenna_count = cursor.read_u16()? as usize;
    let window_count = cursor.read_u16()? as usize;
    let arc_count = cursor.read_u16()? as usize;
    if station_count > 4096 || antenna_count > 8192 || window_count > 16384 || arc_count > 8192 {
        return Err(OrbitError::LimitExceeded("plan counts"));
    }
    let mut stations = HandleArena::default();
    let mut station_handles = Vec::with_capacity(station_count);
    for _ in 0..station_count {
        let handle = stations.insert(GroundStation {
            id: cursor.read_u32()?,
            region: cursor.read_u16()?,
            capacity: cursor.read_u16()?,
            flags: cursor.read_u8()?,
            name_id: cursor.read_u32()?,
        });
        station_handles.push(handle);
    }
    let mut antennas = HandleArena::default();
    let mut antenna_handles = Vec::with_capacity(antenna_count);
    for _ in 0..antenna_count {
        let handle = antennas.insert(Antenna {
            id: cursor.read_u32()?,
            station_id: cursor.read_u32()?,
            band: cursor.read_u8()?,
            gain_db: cursor.read_i16()?,
            slew_rate: cursor.read_u16()?,
        });
        antenna_handles.push(handle);
    }
    let mut windows = Vec::with_capacity(window_count);
    for _ in 0..window_count {
        windows.push(ContactWindow {
            station_id: cursor.read_u32()?,
            satellite_id: cursor.read_u32()?,
            opens_at: cursor.read_u64()?,
            closes_at: cursor.read_u64()?,
            priority: cursor.read_u8()?,
            flags: cursor.read_u8()?,
        });
    }
    let mut arcs = Vec::with_capacity(arc_count);
    for _ in 0..arc_count {
        arcs.push(OrbitArc {
            satellite_id: cursor.read_u32()?,
            epoch: cursor.read_u64()?,
            inclination_mdeg: cursor.read_i32()?,
            altitude_km: cursor.read_u16()?,
            drift_ppm: cursor.read_i16()?,
        });
    }
    Ok(Plan {
        stations,
        antennas,
        station_handles,
        antenna_handles,
        windows,
        arcs,
    })
}

pub fn score_plan(plan: &mut Plan, salt: u64) -> u64 {
    let mut score = salt ^ plan.windows.len() as u64 ^ ((plan.arcs.len() as u64) << 32);
    for station in plan.stations.iter() {
        score ^= station_score(station);
    }
    for antenna in plan.antennas.iter() {
        score = score.wrapping_add(antenna_score(antenna));
    }
    for arc in &plan.arcs {
        score ^= checksum::mix_u64(
            arc.satellite_id as u64
                ^ arc.epoch
                ^ ((arc.altitude_km as u64) << 33)
                ^ (arc.drift_ppm as i64 as u64),
        );
    }
    score ^= arena::relocate_active_handles(&mut plan.stations, score, station_score);
    score ^= arena::handle_window_probe(&mut plan.station_handles, score);
    score ^= conflict_sweep(&mut plan.windows, score);
    score ^ orbit_catalog::score_orbit_catalog(score, plan.windows.len() as u32)
}

fn station_score(station: &GroundStation) -> u64 {
    checksum::mix_u64(
        station.id as u64
            ^ ((station.region as u64) << 17)
            ^ ((station.capacity as u64) << 33)
            ^ ((station.flags as u64) << 49),
    )
}

fn antenna_score(antenna: &Antenna) -> u64 {
    checksum::mix_u64(
        antenna.id as u64
            ^ ((antenna.station_id as u64) << 11)
            ^ ((antenna.band as u64) << 43)
            ^ (antenna.gain_db as i64 as u64),
    )
}

fn window_score(window: &ContactWindow) -> u64 {
    checksum::mix_u64(
        window.station_id as u64
            ^ ((window.satellite_id as u64) << 9)
            ^ window.opens_at.rotate_left((window.priority & 31) as u32)
            ^ window.closes_at
            ^ ((window.flags as u64) << 57),
    )
}

pub fn conflict_sweep(windows: &mut Vec<ContactWindow>, salt: u64) -> u64 {
    if windows.len() < 10 {
        return salt;
    }
    let mut score = salt ^ windows.len() as u64;
    for window in windows.iter() {
        score ^= window_score(window);
    }
    let idx = (checksum::mix_u64(score) as usize) % windows.len();
    let ptr = unsafe { windows.as_ptr().add(idx) };
    if (score & 0x5ff)
        == (((windows[idx].priority as u64) ^ windows[idx].station_id as u64) & 0x5ff)
    {
        windows.retain(|window| window.closes_at > window.opens_at && window.flags & 0x80 == 0);
        windows.shrink_to_fit();
        unsafe {
            score ^= (*ptr).opens_at ^ ((*ptr).satellite_id as u64) << 31;
        }
    }
    score
}

pub fn parse_and_score_plan(data: &[u8]) -> Result<u64> {
    let pool = StringPool::default();
    let mut plan = parse_plan(data, &pool)?;
    Ok(score_plan(&mut plan, 0))
}
