//! Dynamic reticle state machine and simulation engine (§12.3).

use crate::gun::GunProfile;
use crate::input::InputState;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xhair_model::units::Vec2;

/// Discrete simulated character posture and locomotion states.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum PlayerMovementState {
    /// Stationary standing still.
    #[default]
    Idle,
    /// Normal walking speed with directional movement.
    Walking,
    /// Shift walking speed.
    ShiftWalking,
    /// Crouched posture.
    Crouching,
    /// In the air with elapsed airborne timer.
    Airborne {
        /// Elapsed time since jump began in seconds.
        timer: f32,
    },
}

/// Dynamic drivers produced by the simulation engine to modulate crosshair properties (§12.1).
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, JsonSchema)]
pub struct DynamicDrivers {
    /// Overall calculated weapon spread in game pixels.
    pub spread: f32,
    /// Normalized movement speed from 0.0 (idle) to 1.0 (running).
    pub movement_speed: f32,
    /// Firing heat and cadence from 0.0 (cold) to 1.0 (sustained spray).
    pub firing_heat: f32,
    /// Current recoil climb offset in pixels.
    pub recoil_offset: Vec2,
    /// Current shot index during continuous fire.
    pub shot_count: u32,
    /// Whether character is currently airborne.
    pub is_airborne: bool,
    /// Whether character is currently crouched.
    pub is_crouched: bool,
}

/// Internal simulation state managing timers, recoil climb, and spread recovery.
#[derive(Debug, Clone, Default)]
pub struct SimulationEngine {
    movement_state: PlayerMovementState,
    current_spread: f32,
    target_spread: f32,
    current_recoil: Vec2,
    shot_count: u32,
    time_since_last_shot: f32,
    firing_heat: f32,
    airborne_timer: f32,
}

impl SimulationEngine {
    /// Creates a new simulation engine instance in idle state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Resets all dynamic states, timers, and recoil offsets to baseline.
    pub fn reset(&mut self) {
        *self = Self::default();
    }

    /// Advances the simulation by time `dt` seconds given current user input and gun profile.
    pub fn step(&mut self, input: &InputState, profile: &GunProfile, dt: f32) -> DynamicDrivers {
        // 1. Update movement state & airborne physics
        if input.jump {
            self.movement_state = PlayerMovementState::Airborne {
                timer: self.airborne_timer,
            };
            self.airborne_timer += dt;
            if self.airborne_timer > 0.65 {
                // Land after standard jump arc
                self.airborne_timer = 0.0;
            }
        } else {
            self.airborne_timer = 0.0;
            if input.crouching {
                self.movement_state = PlayerMovementState::Crouching;
            } else if input.is_moving() {
                if input.walking_shift {
                    self.movement_state = PlayerMovementState::ShiftWalking;
                } else {
                    self.movement_state = PlayerMovementState::Walking;
                }
            } else {
                self.movement_state = PlayerMovementState::Idle;
            }
        }

        // 2. Movement penalties & bonuses
        let mut loc_spread = profile.spread.base;
        let mut is_airborne = false;
        let mut is_crouched = false;

        match self.movement_state {
            PlayerMovementState::Idle => {}
            PlayerMovementState::Walking => loc_spread += profile.spread.move_penalty,
            PlayerMovementState::ShiftWalking => loc_spread += profile.spread.shift_move_penalty,
            PlayerMovementState::Crouching => {
                is_crouched = true;
                loc_spread = (loc_spread - profile.spread.crouch_bonus).max(0.2);
            }
            PlayerMovementState::Airborne { .. } => {
                is_airborne = true;
                loc_spread += profile.spread.air_penalty;
            }
        }

        // 3. Firing cadence and recoil calculation
        let shot_interval = if profile.fire_rate_hz > 0.0 {
            1.0 / profile.fire_rate_hz
        } else {
            0.1
        };

        self.time_since_last_shot += dt;

        if input.fire {
            // Trigger shots at weapon cyclic rate
            if self.time_since_last_shot >= shot_interval {
                self.shot_count = (self.shot_count + 1).min(profile.magazine);
                self.time_since_last_shot = 0.0;
            }

            self.firing_heat = (self.firing_heat + dt * 4.0).min(1.0);
        } else {
            // Weapon recovery
            if self.time_since_last_shot > shot_interval * 1.5 {
                self.shot_count = 0;
            }
            self.firing_heat = (self.firing_heat - dt * 3.0).max(0.0);
        }

        // Target spread combines movement and continuous firing penalties
        let fire_penalty = (self.shot_count as f32) * profile.spread.per_shot;
        self.target_spread = (loc_spread + fire_penalty).min(profile.spread.max);

        // 4. Smooth spread transitions (attack/release smoothing)
        let smooth_rate = if self.target_spread > self.current_spread {
            1000.0 / profile.smoothing.attack_ms.max(1.0)
        } else {
            1000.0 / profile.smoothing.release_ms.max(1.0)
        };
        let spread_delta = (self.target_spread - self.current_spread) * (smooth_rate * dt).min(1.0);
        self.current_spread += spread_delta;

        // 5. Calculate recoil climb offset from pattern
        let target_recoil = if self.shot_count > 0 && !profile.pattern.is_empty() {
            let idx = (self.shot_count - 1) as usize;
            if idx < profile.pattern.len() {
                profile.pattern[idx]
            } else {
                *profile.pattern.last().unwrap()
            }
        } else {
            Vec2::ZERO
        };

        // Smooth recoil recovery
        let recoil_rate = (15.0 * dt).min(1.0);
        self.current_recoil.x += (target_recoil.x - self.current_recoil.x) * recoil_rate;
        self.current_recoil.y += (target_recoil.y - self.current_recoil.y) * recoil_rate;

        DynamicDrivers {
            spread: self.current_spread,
            movement_speed: input.movement_speed(),
            firing_heat: self.firing_heat,
            recoil_offset: self.current_recoil,
            shot_count: self.shot_count,
            is_airborne,
            is_crouched,
        }
    }
}
