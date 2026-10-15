//! Restricted input sources and state abstractions for dynamic reticles (§12.2).
//!
//! Compliance rule (§2):
//! Restrict to user-bound keys/buttons only. Discard everything else at the earliest point.
//! Zero keystroke logging, zero input persistence, zero network calls.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// High-level user movement and firing state for dynamic simulation.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct InputState {
    /// Moving forward (e.g. W key).
    pub forward: bool,
    /// Moving backward (e.g. S key).
    pub backward: bool,
    /// Moving left (e.g. A key).
    pub left: bool,
    /// Moving right (e.g. D key).
    pub right: bool,
    /// Walking modifier (e.g. Left Shift).
    pub walking_shift: bool,
    /// Crouching modifier (e.g. Left Ctrl).
    pub crouching: bool,
    /// Airborne jumping trigger (e.g. Space).
    pub jump: bool,
    /// Primary fire button held (e.g. Left Mouse Button).
    pub fire: bool,
}

impl InputState {
    /// Returns true if any directional movement key is pressed.
    #[inline]
    pub fn is_moving(&self) -> bool {
        self.forward || self.backward || self.left || self.right
    }

    /// Returns movement intensity factor: 0.0 = stationary, 0.5 = shift walking, 1.0 = running.
    #[inline]
    pub fn movement_speed(&self) -> f32 {
        if !self.is_moving() {
            0.0
        } else if self.walking_shift {
            0.5
        } else {
            1.0
        }
    }
}

/// Abstract input source trait allowing live Raw Input or synthetic test sequences (§12.2).
pub trait InputSource: Send + Sync {
    /// Polls the current state of bound movement and fire inputs.
    fn poll_state(&mut self) -> InputState;
}

/// Synthetic input playback source for deterministic unit testing.
#[derive(Debug, Clone, Default)]
pub struct SyntheticInputSource {
    sequence: Vec<(f32, InputState)>,
    current_index: usize,
    elapsed_time: f32,
}

impl SyntheticInputSource {
    /// Creates an empty synthetic input source.
    pub fn new() -> Self {
        Self::default()
    }

    /// Queues an input state to become active at timestamp `at_seconds`.
    pub fn queue_state(&mut self, at_seconds: f32, state: InputState) {
        self.sequence.push((at_seconds, state));
        self.sequence
            .sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    }

    /// Advances the simulation clock by `dt` seconds.
    pub fn advance_time(&mut self, dt: f32) {
        self.elapsed_time += dt;
        while self.current_index + 1 < self.sequence.len()
            && self.sequence[self.current_index + 1].0 <= self.elapsed_time
        {
            self.current_index += 1;
        }
    }
}

impl InputSource for SyntheticInputSource {
    fn poll_state(&mut self) -> InputState {
        if self.sequence.is_empty() {
            InputState::default()
        } else {
            self.sequence[self.current_index].1
        }
    }
}
