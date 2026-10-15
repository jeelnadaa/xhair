//! Dynamic property bindings mapping simulation drivers to crosshair properties (§12.5).

use crate::state_machine::DynamicDrivers;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xhair_model::crosshair::Crosshair;
use xhair_model::shapes::LayerKind;

/// Source driver channel produced by the simulation engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DriverSource {
    /// Overall weapon spread (movement + fire penalty).
    Spread,
    /// Normalized movement speed (0.0=idle, 0.5=shift, 1.0=run).
    MovementSpeed,
    /// Firing cadence / heat (0.0 to 1.0).
    FiringHeat,
    /// Recoil horizontal kick offset in pixels.
    RecoilX,
    /// Recoil vertical climb offset in pixels.
    RecoilY,
}

/// Curve modulation transfer function.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(tag = "curve", rename_all = "snake_case")]
pub enum BindingCurve {
    /// Linear 1:1 ramp.
    #[default]
    Linear,
    /// Smooth ease in ($t^2$).
    EaseIn,
    /// Smooth ease out ($1 - (1-t)^2$).
    EaseOut,
    /// Symmetric ease in-out ($3t^2 - 2t^3$).
    EaseInOut,
}

/// Operation mode for applying driver modulation to target crosshair property.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "snake_case")]
pub enum BindingMode {
    /// Add modulated offset to baseline property (`prop += delta`).
    #[default]
    Add,
    /// Set absolute property directly (`prop = val`).
    Set,
    /// Scale property proportionally (`prop *= factor`).
    Multiply,
}

/// Dynamic binding linking a simulation driver to a crosshair property (§12.5).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DynamicBinding {
    /// Unique identifier for this binding.
    pub id: String,
    /// Path to target property (e.g. `layers[0].gap`, `global.scale`, `layers[1].opacity`).
    pub property_path: String,
    /// Driver channel providing input value.
    pub driver: DriverSource,
    /// Input driver domain range `(in_min, in_max)`.
    pub in_range: (f32, f32),
    /// Output target range `(out_min, out_max)`.
    pub out_range: (f32, f32),
    /// Transfer function curve.
    pub curve: BindingCurve,
    /// Modulation application mode.
    pub mode: BindingMode,
}

impl DynamicBinding {
    /// Evaluates the binding against active simulation drivers.
    pub fn evaluate(&self, drivers: &DynamicDrivers) -> f32 {
        let raw_val = match self.driver {
            DriverSource::Spread => drivers.spread,
            DriverSource::MovementSpeed => drivers.movement_speed,
            DriverSource::FiringHeat => drivers.firing_heat,
            DriverSource::RecoilX => drivers.recoil_offset.x,
            DriverSource::RecoilY => drivers.recoil_offset.y,
        };

        // Normalize to 0.0 .. 1.0 within in_range
        let (in_min, in_max) = self.in_range;
        let in_span = (in_max - in_min).max(1e-4);
        let t = ((raw_val - in_min) / in_span).clamp(0.0, 1.0);

        // Apply transfer curve
        let curved_t = match self.curve {
            BindingCurve::Linear => t,
            BindingCurve::EaseIn => t * t,
            BindingCurve::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            BindingCurve::EaseInOut => t * t * (3.0 - 2.0 * t),
        };

        // Map to output range
        let (out_min, out_max) = self.out_range;
        out_min + curved_t * (out_max - out_min)
    }
}

/// Modulates a crosshair's properties in-place according to active dynamic bindings (§12.5).
pub fn apply_bindings(
    crosshair: &mut Crosshair,
    bindings: &[DynamicBinding],
    drivers: &DynamicDrivers,
) {
    for binding in bindings {
        let val = binding.evaluate(drivers);
        let path = binding.property_path.as_str();

        if path == "global.scale" {
            match binding.mode {
                BindingMode::Add => crosshair.global.scale += val,
                BindingMode::Set => crosshair.global.scale = val,
                BindingMode::Multiply => crosshair.global.scale *= val,
            }
        } else if path == "global.opacity" {
            match binding.mode {
                BindingMode::Add => {
                    crosshair.global.opacity =
                        xhair_model::units::Alpha::from(crosshair.global.opacity.0 + val)
                }
                BindingMode::Set => crosshair.global.opacity = xhair_model::units::Alpha::from(val),
                BindingMode::Multiply => {
                    crosshair.global.opacity =
                        xhair_model::units::Alpha::from(crosshair.global.opacity.0 * val)
                }
            }
        } else if path == "recoil.offset" {
            crosshair.global.offset.x += drivers.recoil_offset.x;
            crosshair.global.offset.y += drivers.recoil_offset.y;
        } else if path.starts_with("layers[") {
            // Parse layer index
            if let Some(end_bracket) = path.find(']') {
                if let Ok(idx) = path[7..end_bracket].parse::<usize>() {
                    if let Some(layer) = crosshair.layers.get_mut(idx) {
                        let prop = &path[end_bracket + 2..]; // skip "]."
                        match prop {
                            "gap" => {
                                if let LayerKind::Line(line) = &mut layer.kind {
                                    match binding.mode {
                                        BindingMode::Add => line.gap += val,
                                        BindingMode::Set => line.gap = val,
                                        BindingMode::Multiply => line.gap *= val,
                                    }
                                }
                            }
                            "thickness" => {
                                if let LayerKind::Line(line) = &mut layer.kind {
                                    match binding.mode {
                                        BindingMode::Add => {
                                            line.thickness = (line.thickness + val).max(0.5)
                                        }
                                        BindingMode::Set => line.thickness = val.max(0.5),
                                        BindingMode::Multiply => {
                                            line.thickness = (line.thickness * val).max(0.5)
                                        }
                                    }
                                }
                            }
                            "opacity" => match binding.mode {
                                BindingMode::Add => {
                                    layer.opacity =
                                        xhair_model::units::Alpha::from(layer.opacity.0 + val)
                                }
                                BindingMode::Set => {
                                    layer.opacity = xhair_model::units::Alpha::from(val)
                                }
                                BindingMode::Multiply => {
                                    layer.opacity =
                                        xhair_model::units::Alpha::from(layer.opacity.0 * val)
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
