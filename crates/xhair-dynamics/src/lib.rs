//! Dynamic reticle pure state simulation and spread curves for xhair.
//!
//! # Architecture & Safety (§2, §12)
//!
//! - **Pure simulation:** `xhair-dynamics` is completely detached from game memory and display pipelines.
//! - **Inputs:** Accepts high-level locomotion and firing states via [`InputSource`] or synthetic sequences.
//! - **Drivers:** Produces [`DynamicDrivers`] (spread, movement speed, recoil offsets) to modulate crosshair properties.
//! - **Compliance:** Gun profiles contain example values only. No game-matching recoil values are shipped.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod binding;
pub mod gun;
pub mod input;
pub mod simulator;
pub mod state_machine;

// Re-exports
pub use binding::{BindingCurve, BindingMode, DriverSource, DynamicBinding};
pub use gun::{GunProfile, SmoothingConfig, SpreadConfig};
pub use input::{InputSource, InputState, SyntheticInputSource};
pub use simulator::{simulate_sequence, simulate_spray, SprayShotImpact, SpraySimulationResult};
pub use state_machine::{DynamicDrivers, PlayerMovementState, SimulationEngine};

/// Maximum simulation tick rate in Hz (§12.7).
pub const MAX_SIMULATION_HZ: u32 = 120;

#[cfg(test)]
mod tests {
    use super::*;
    use xhair_model::crosshair::Crosshair;

    #[test]
    fn test_simulation_rate_limit() {
        assert_eq!(MAX_SIMULATION_HZ, 120);
    }

    #[test]
    fn test_gun_templates() {
        let rifle = GunProfile::template_rifle();
        assert_eq!(rifle.name, "Rifle (Template)");
        assert_eq!(rifle.magazine, 30);
        assert_eq!(rifle.pattern.len(), 30);

        let smg = GunProfile::template_smg();
        assert_eq!(smg.name, "SMG (Template)");
        assert_eq!(smg.magazine, 30);
        assert_eq!(smg.pattern.len(), 30);

        let pistol = GunProfile::template_pistol();
        assert_eq!(pistol.name, "Pistol (Template)");
        assert_eq!(pistol.magazine, 12);
        assert_eq!(pistol.pattern.len(), 12);
    }

    #[test]
    fn test_input_movement_speed() {
        let mut input = InputState::default();
        assert_eq!(input.movement_speed(), 0.0);
        assert!(!input.is_moving());

        input.forward = true;
        assert_eq!(input.movement_speed(), 1.0);
        assert!(input.is_moving());

        input.walking_shift = true;
        assert_eq!(input.movement_speed(), 0.5);
    }

    #[test]
    fn test_state_machine_movement_penalties() {
        let profile = GunProfile::template_rifle();
        let mut engine = SimulationEngine::new();

        // 1. Idle state
        let idle_input = InputState::default();
        let idle_drivers = engine.step(&idle_input, &profile, 0.5);
        assert!((idle_drivers.spread - profile.spread.base).abs() < 0.1);
        assert_eq!(idle_drivers.movement_speed, 0.0);

        // 2. Walking penalty
        let walk_input = InputState {
            forward: true,
            ..Default::default()
        };
        let walk_drivers = engine.step(&walk_input, &profile, 0.5);
        assert!(walk_drivers.spread > idle_drivers.spread);

        // 3. Crouching bonus
        let crouch_input = InputState {
            crouching: true,
            ..Default::default()
        };
        let crouch_drivers = engine.step(&crouch_input, &profile, 0.5);
        assert!(crouch_drivers.is_crouched);
        assert!(crouch_drivers.spread < walk_drivers.spread);
    }

    #[test]
    fn test_state_machine_firing_and_recovery() {
        let profile = GunProfile::template_rifle();
        let mut engine = SimulationEngine::new();

        let fire_input = InputState {
            fire: true,
            ..Default::default()
        };

        // Fire multiple ticks
        for _ in 0..10 {
            engine.step(&fire_input, &profile, 0.1);
        }
        let fired_drivers = engine.step(&fire_input, &profile, 0.1);
        assert!(fired_drivers.shot_count > 0);
        assert!(fired_drivers.firing_heat > 0.0);
        assert!(fired_drivers.spread > profile.spread.base);

        // Cease fire and observe recovery
        let idle_input = InputState::default();
        for _ in 0..30 {
            engine.step(&idle_input, &profile, 0.1);
        }
        let recovered_drivers = engine.step(&idle_input, &profile, 0.1);
        assert_eq!(recovered_drivers.shot_count, 0);
        assert_eq!(recovered_drivers.firing_heat, 0.0);
    }

    #[test]
    fn test_dynamic_binding_evaluation() {
        let binding = DynamicBinding {
            id: "test_spread_gap".into(),
            property_path: "layers[0].gap".into(),
            driver: DriverSource::Spread,
            in_range: (1.0, 10.0),
            out_range: (2.0, 15.0),
            curve: BindingCurve::Linear,
            mode: BindingMode::Add,
        };

        let mut drivers = DynamicDrivers {
            spread: 1.0,
            ..Default::default()
        };
        assert_eq!(binding.evaluate(&drivers), 2.0);

        drivers.spread = 10.0;
        assert_eq!(binding.evaluate(&drivers), 15.0);

        drivers.spread = 5.5;
        assert!((binding.evaluate(&drivers) - 8.5).abs() < 1e-3);
    }

    #[test]
    fn test_simulate_spray() {
        let profile = GunProfile::template_smg();
        let result = simulate_spray(&profile, Some(10));

        assert_eq!(result.profile_id, profile.id);
        assert_eq!(result.shots.len(), 10);
        assert!(result.total_duration_seconds > 0.0);
        assert!(result.max_spread >= profile.spread.base);
    }

    #[test]
    fn test_apply_bindings_to_crosshair() {
        let mut xhair = Crosshair::new_classic("test");
        let base_gap = match &xhair.layers[0].kind {
            xhair_model::shapes::LayerKind::Line(line) => line.gap,
            _ => 0.0,
        };

        let binding = DynamicBinding {
            id: "bind_gap".into(),
            property_path: "layers[0].gap".into(),
            driver: DriverSource::Spread,
            in_range: (0.0, 10.0),
            out_range: (0.0, 5.0),
            curve: BindingCurve::Linear,
            mode: BindingMode::Add,
        };

        let drivers = DynamicDrivers {
            spread: 10.0, // out_val = 5.0
            ..Default::default()
        };

        binding::apply_bindings(&mut xhair, &[binding], &drivers);

        let new_gap = match &xhair.layers[0].kind {
            xhair_model::shapes::LayerKind::Line(line) => line.gap,
            _ => 0.0,
        };

        assert!((new_gap - (base_gap + 5.0)).abs() < 1e-4);
    }
}
