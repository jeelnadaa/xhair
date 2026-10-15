//! Gun profiles, custom spread curves, and recoil patterns (§12.4).
//!
//! Important compliance notice (§2, §12.4):
//! Gun values are user-entered and simulated. The app never ships numbers claiming to match real game recoil.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xhair_model::units::Vec2;

/// Weapon spread penalty parameters and recovery dynamics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SpreadConfig {
    /// Base idle standing spread in game pixels / degrees.
    pub base: f32,
    /// Spread added per continuous shot fired.
    pub per_shot: f32,
    /// Maximum spread ceiling.
    pub max: f32,
    /// Spread recovery rate per second once firing ceases.
    pub recovery_per_sec: f32,
    /// Flat penalty added during normal walking.
    pub move_penalty: f32,
    /// Flat penalty added during shift walking.
    pub shift_move_penalty: f32,
    /// Spread reduction bonus when crouched.
    pub crouch_bonus: f32,
    /// Flat penalty added when airborne.
    pub air_penalty: f32,
}

impl Default for SpreadConfig {
    fn default() -> Self {
        Self {
            base: 1.0,
            per_shot: 0.8,
            max: 12.0,
            recovery_per_sec: 15.0,
            move_penalty: 4.0,
            shift_move_penalty: 1.5,
            crouch_bonus: 0.4,
            air_penalty: 8.0,
        }
    }
}

/// Attack and release smoothing durations in milliseconds.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SmoothingConfig {
    /// Attack duration in milliseconds when driver value increases.
    pub attack_ms: f32,
    /// Release duration in milliseconds when driver value recovers.
    pub release_ms: f32,
}

impl Default for SmoothingConfig {
    fn default() -> Self {
        Self {
            attack_ms: 20.0,
            release_ms: 120.0,
        }
    }
}

/// Gun profile configuring simulated recoil and spread transitions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GunProfile {
    /// Unique profile identifier.
    pub id: String,
    /// Profile display name.
    pub name: String,
    /// User notes and description (labeled as example/simulated).
    pub notes: String,
    /// Sustained cyclic fire rate in Hertz.
    pub fire_rate_hz: f32,
    /// Total magazine round capacity.
    pub magazine: u32,
    /// Spread dynamics configuration.
    pub spread: SpreadConfig,
    /// Temporal smoothing configuration.
    pub smoothing: SmoothingConfig,
    /// Recoil pattern offsets per sequential shot (x=horizontal, y=vertical kick in pixels).
    pub pattern: Vec<Vec2>,
}

impl GunProfile {
    /// Generic rifle template with typical upward climb and lateral sway (example only).
    pub fn template_rifle() -> Self {
        let mut pattern = Vec::with_capacity(30);
        for i in 0..30 {
            let t = i as f32;
            let kick_y = (t * 2.5).min(28.0);
            let kick_x = if i < 8 {
                0.0
            } else if i < 18 {
                (i - 8) as f32 * 1.4
            } else {
                14.0 - ((i - 18) as f32 * 2.2)
            };
            pattern.push(Vec2::new(kick_x, kick_y));
        }

        Self {
            id: "gun-template-rifle".to_string(),
            name: "Rifle (Template)".to_string(),
            notes: "Example placeholder curve - not real game data (§12.4)".to_string(),
            fire_rate_hz: 11.0,
            magazine: 30,
            spread: SpreadConfig {
                base: 1.2,
                per_shot: 0.9,
                max: 14.0,
                recovery_per_sec: 18.0,
                move_penalty: 5.0,
                shift_move_penalty: 2.0,
                crouch_bonus: 0.5,
                air_penalty: 9.0,
            },
            smoothing: SmoothingConfig::default(),
            pattern,
        }
    }

    /// High fire-rate compact weapon template with wide lateral spread.
    pub fn template_smg() -> Self {
        let mut pattern = Vec::with_capacity(30);
        for i in 0..30 {
            let kick_y = (i as f32 * 1.8).min(20.0);
            let kick_x = ((i as f32 * 1.2).sin()) * 8.0;
            pattern.push(Vec2::new(kick_x, kick_y));
        }

        Self {
            id: "gun-template-smg".to_string(),
            name: "SMG (Template)".to_string(),
            notes: "Example placeholder curve - not real game data (§12.4)".to_string(),
            fire_rate_hz: 15.0,
            magazine: 30,
            spread: SpreadConfig {
                base: 1.8,
                per_shot: 0.6,
                max: 18.0,
                recovery_per_sec: 22.0,
                move_penalty: 3.5,
                shift_move_penalty: 1.2,
                crouch_bonus: 0.3,
                air_penalty: 7.0,
            },
            smoothing: SmoothingConfig {
                attack_ms: 10.0,
                release_ms: 100.0,
            },
            pattern,
        }
    }

    /// Semi-automatic pistol template with quick recovery.
    pub fn template_pistol() -> Self {
        let mut pattern = Vec::with_capacity(12);
        for i in 0..12 {
            pattern.push(Vec2::new(0.0, (i as f32 * 3.0).min(15.0)));
        }

        Self {
            id: "gun-template-pistol".to_string(),
            name: "Pistol (Template)".to_string(),
            notes: "Example placeholder curve - not real game data (§12.4)".to_string(),
            fire_rate_hz: 6.0,
            magazine: 12,
            spread: SpreadConfig {
                base: 0.8,
                per_shot: 1.4,
                max: 10.0,
                recovery_per_sec: 25.0,
                move_penalty: 2.0,
                shift_move_penalty: 0.8,
                crouch_bonus: 0.2,
                air_penalty: 6.0,
            },
            smoothing: SmoothingConfig {
                attack_ms: 15.0,
                release_ms: 80.0,
            },
            pattern,
        }
    }
}
