use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrbitError {
    Empty,
    BadMagic,
    BadVersion(u8),
    ShortRead { needed: usize, remaining: usize },
    LimitExceeded(&'static str),
    BadSegment(&'static str),
    BadPool(&'static str),
    BadPlan(&'static str),
    BadTelemetry(&'static str),
    BadScript(&'static str),
}

pub type Result<T> = std::result::Result<T, OrbitError>;

impl fmt::Display for OrbitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrbitError::Empty => write!(f, "empty input"),
            OrbitError::BadMagic => write!(f, "bad magic"),
            OrbitError::BadVersion(v) => write!(f, "unsupported version {v}"),
            OrbitError::ShortRead { needed, remaining } => {
                write!(f, "short read: needed {needed}, remaining {remaining}")
            }
            OrbitError::LimitExceeded(name) => write!(f, "limit exceeded: {name}"),
            OrbitError::BadSegment(name) => write!(f, "bad segment: {name}"),
            OrbitError::BadPool(name) => write!(f, "bad string pool: {name}"),
            OrbitError::BadPlan(name) => write!(f, "bad contact plan: {name}"),
            OrbitError::BadTelemetry(name) => write!(f, "bad telemetry: {name}"),
            OrbitError::BadScript(name) => write!(f, "bad script: {name}"),
        }
    }
}

impl std::error::Error for OrbitError {}
