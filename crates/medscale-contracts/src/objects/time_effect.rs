//! Medical time, effect states, and placement classes.

use serde::{Deserialize, Serialize};

/// Precision of a medical/recorded time value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimePrecision {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Instant,
    Unknown,
}

/// Structured time with explicit precision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MedicalTime {
    /// RFC3339-ish or partial timestamp string (logical; validated by callers).
    pub value: String,
    pub precision: TimePrecision,
    pub approximate: bool,
}

/// External-action / effect state machine vocabulary (Spec 014 hardens durability).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectState {
    Pending,
    Sent,
    Confirmed,
    Failed,
    Unknown,
}

/// OSS matrix placement class for native/FFI components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlacementClass {
    P0,
    P1,
    P2,
    P3,
}
