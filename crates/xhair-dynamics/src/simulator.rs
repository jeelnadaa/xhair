//! Standalone pure simulation engine for spray patterns and dynamic response (§12.6).
//!
//! Important safety notice (§2, §12.6):
//! All patterns and dynamics are simulated within this crate. No memory reads or screen capture.

use crate::gun::GunProfile;
use crate::input::InputState;
use crate::state_machine::{DynamicDrivers, SimulationEngine};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xhair_model::units::Vec2;

/// Simulated point of impact and spread circle for a single shot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SprayShotImpact {
    /// Zero-based shot index within current spray (0 .. magazine).
    pub shot_index: u32,
    /// Recoil center offset in pixels (x=horizontal sway, y=vertical climb).
    pub position: Vec2,
    /// Timestamp relative to the first shot in seconds.
    pub timestamp_seconds: f32,
    /// Spread radius / error disc around the center at the time of firing.
    pub spread_radius: f32,
}

/// Aggregated result of a simulated weapon spray burst (§12.6).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SpraySimulationResult {
    /// Associated gun profile identifier.
    pub profile_id: String,
    /// Sequential shot impacts for the entire burst.
    pub shots: Vec<SprayShotImpact>,
    /// Total duration of the simulated burst in seconds.
    pub total_duration_seconds: f32,
    /// Maximum spread radius observed across the burst.
    pub max_spread: f32,
    /// Minimum bounding rectangle corner (min_x, min_y).
    pub bounds_min: Vec2,
    /// Maximum bounding rectangle corner (max_x, max_y).
    pub bounds_max: Vec2,
}

/// Simulates a continuous firing spray using the provided gun profile.
///
/// Returns the sequence of impact locations, timestamps, and spread radii.
pub fn simulate_spray(profile: &GunProfile, max_shots: Option<u32>) -> SpraySimulationResult {
    let shots_to_fire = max_shots.unwrap_or(profile.magazine).min(profile.magazine);
    let shot_interval = if profile.fire_rate_hz > 0.0 {
        1.0 / profile.fire_rate_hz
    } else {
        0.1
    };

    let mut engine = SimulationEngine::new();
    let mut shots = Vec::with_capacity(shots_to_fire as usize);

    let mut min_x = 0.0f32;
    let mut max_x = 0.0f32;
    let mut min_y = 0.0f32;
    let mut max_y = 0.0f32;
    let mut max_spread = 0.0f32;

    // Simulation runs at high internal fidelity (120 Hz)
    let sim_dt = 1.0 / 120.0;
    let mut current_time = 0.0;
    let mut next_shot_time = 0.0;
    let mut shot_index = 0;

    let firing_input = InputState {
        fire: true,
        ..Default::default()
    };

    while shot_index < shots_to_fire {
        let drivers = engine.step(&firing_input, profile, sim_dt);
        current_time += sim_dt;

        if current_time >= next_shot_time {
            let offset = if (shot_index as usize) < profile.pattern.len() {
                profile.pattern[shot_index as usize]
            } else if !profile.pattern.is_empty() {
                *profile.pattern.last().unwrap()
            } else {
                Vec2::new(0.0, shot_index as f32 * 2.0)
            };

            let impact = SprayShotImpact {
                shot_index,
                position: offset,
                timestamp_seconds: next_shot_time,
                spread_radius: drivers.spread,
            };

            min_x = min_x.min(offset.x - drivers.spread);
            max_x = max_x.max(offset.x + drivers.spread);
            min_y = min_y.min(offset.y - drivers.spread);
            max_y = max_y.max(offset.y + drivers.spread);
            max_spread = max_spread.max(drivers.spread);

            shots.push(impact);
            shot_index += 1;
            next_shot_time += shot_interval;
        }
    }

    SpraySimulationResult {
        profile_id: profile.id.clone(),
        shots,
        total_duration_seconds: (shots_to_fire as f32) * shot_interval,
        max_spread,
        bounds_min: Vec2::new(min_x, min_y),
        bounds_max: Vec2::new(max_x, max_y),
    }
}

/// Simulates a synthetic timeline sequence of inputs and returns driver frames at each step.
pub fn simulate_sequence(
    profile: &GunProfile,
    steps: &[(f32, InputState)],
    step_dt: f32,
) -> Vec<(f32, DynamicDrivers)> {
    let mut engine = SimulationEngine::new();
    let mut results = Vec::new();

    let mut step_idx = 0;
    let mut current_input = InputState::default();
    let total_time = steps.last().map(|(t, _)| *t).unwrap_or(1.0);
    let dt = if step_dt <= 0.0 { 1.0 / 60.0 } else { step_dt };

    let mut t = 0.0;
    while t <= total_time {
        while step_idx < steps.len() && steps[step_idx].0 <= t {
            current_input = steps[step_idx].1;
            step_idx += 1;
        }

        let drivers = engine.step(&current_input, profile, dt);
        results.push((t, drivers));
        t += dt;
    }

    results
}
