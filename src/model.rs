use crate::arena::{Handle, HandleArena};

#[derive(Clone, Debug, Default)]
pub struct Header {
    pub version: u8,
    pub flags: u8,
    pub captured_at: u64,
    pub mission_id: u32,
    pub segment_count: u16,
}

#[derive(Clone, Debug)]
pub struct Segment {
    pub kind: u8,
    pub flags: u8,
    pub lane: u16,
    pub offset: usize,
    pub len: usize,
    pub checksum: u32,
    pub tag: u16,
}

#[derive(Clone, Debug, Default)]
pub struct StringPool {
    pub pages: Vec<Vec<u8>>,
    pub aliases: Vec<Alias>,
    pub digest: u64,
}

#[derive(Clone, Debug)]
pub struct Alias {
    pub id: u32,
    pub page: usize,
    pub offset: usize,
    pub len: usize,
    pub weight: u32,
}

#[derive(Clone, Debug, Default)]
pub struct GroundStation {
    pub id: u32,
    pub region: u16,
    pub capacity: u16,
    pub flags: u8,
    pub name_id: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Antenna {
    pub id: u32,
    pub station_id: u32,
    pub band: u8,
    pub gain_db: i16,
    pub slew_rate: u16,
}

#[derive(Clone, Debug, Default)]
pub struct ContactWindow {
    pub station_id: u32,
    pub satellite_id: u32,
    pub opens_at: u64,
    pub closes_at: u64,
    pub priority: u8,
    pub flags: u8,
}

#[derive(Clone, Debug, Default)]
pub struct OrbitArc {
    pub satellite_id: u32,
    pub epoch: u64,
    pub inclination_mdeg: i32,
    pub altitude_km: u16,
    pub drift_ppm: i16,
}

#[derive(Clone, Debug, Default)]
pub struct Plan {
    pub stations: HandleArena<GroundStation>,
    pub antennas: HandleArena<Antenna>,
    pub station_handles: Vec<Handle>,
    pub antenna_handles: Vec<Handle>,
    pub windows: Vec<ContactWindow>,
    pub arcs: Vec<OrbitArc>,
}

#[derive(Clone, Debug)]
pub enum TelemetryFrame {
    Beacon {
        satellite_id: u32,
        status: u32,
        at: u64,
    },
    DeltaSeries {
        channel: u16,
        samples: Vec<i16>,
        at: u64,
    },
    StateVector {
        satellite_id: u32,
        x: i32,
        y: i32,
        z: i32,
        at: u64,
    },
    Alarm {
        code: u16,
        severity: u8,
        text: Vec<u8>,
        at: u64,
    },
    Opaque {
        kind: u8,
        payload: Vec<u8>,
        at: u64,
    },
}

impl TelemetryFrame {
    pub fn at(&self) -> u64 {
        match self {
            TelemetryFrame::Beacon { at, .. }
            | TelemetryFrame::DeltaSeries { at, .. }
            | TelemetryFrame::StateVector { at, .. }
            | TelemetryFrame::Alarm { at, .. }
            | TelemetryFrame::Opaque { at, .. } => *at,
        }
    }

    pub fn code(&self) -> u8 {
        match self {
            TelemetryFrame::Beacon { .. } => 1,
            TelemetryFrame::DeltaSeries { .. } => 2,
            TelemetryFrame::StateVector { .. } => 3,
            TelemetryFrame::Alarm { .. } => 4,
            TelemetryFrame::Opaque { kind, .. } => *kind,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TelemetrySet {
    pub stream_id: u32,
    pub started_at: u64,
    pub frames: Vec<TelemetryFrame>,
    pub digest: u64,
}

#[derive(Clone, Debug)]
pub struct EventRecord {
    pub kind: u8,
    pub satellite_id: u32,
    pub clock: u64,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct ScriptProgram {
    pub version: u8,
    pub locals: Vec<i64>,
    pub symbols: Vec<u32>,
    pub instructions: Vec<Instruction>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Instruction {
    pub opcode: u8,
    pub operand: i32,
    pub line: u32,
}

#[derive(Clone, Debug, Default)]
pub struct Bundle {
    pub header: Header,
    pub segments: Vec<Segment>,
    pub strings: StringPool,
    pub plan: Plan,
    pub telemetry: Vec<TelemetrySet>,
    pub programs: Vec<ScriptProgram>,
    pub events: Vec<EventRecord>,
    pub blobs: Vec<Vec<u8>>,
    pub diagnostics: Vec<String>,
}

impl Bundle {
    pub fn new(header: Header, segments: Vec<Segment>) -> Self {
        Self {
            header,
            segments,
            ..Self::default()
        }
    }

    pub fn frame_count(&self) -> usize {
        self.telemetry.iter().map(|set| set.frames.len()).sum()
    }
}

#[derive(Clone, Debug, Default)]
pub struct ScheduleReport {
    pub findings: Vec<ScheduleFinding>,
    pub plan_score: u64,
    pub telemetry_score: u64,
    pub event_score: u64,
    pub script_score: u64,
    pub catalog_score: u64,
    pub frame_count: usize,
}

#[derive(Clone, Debug)]
pub struct ScheduleFinding {
    pub severity: u8,
    pub code: String,
    pub detail: u64,
}
