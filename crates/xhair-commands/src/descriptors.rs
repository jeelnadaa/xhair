//! Schema-driven property descriptors: single source of truth for UI controls, tooltips, and MCP schemas.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Detailed metadata describing a single editable property.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PropertyDescriptor {
    /// Canonical property path or key (e.g. "length", "thickness", "gap").
    pub name: String,
    /// Human-facing display label in the inspector.
    pub label: String,
    /// Mandatory plain-language tooltip explanation (Requirement P5, §5.3).
    pub tip: String,
    /// Engineering unit (e.g. "px", "deg", "%").
    pub unit: Option<String>,
    /// Minimum permissible numeric value.
    pub min: Option<f64>,
    /// Maximum permissible numeric value.
    pub max: Option<f64>,
    /// Standard slider/spinner step amount.
    pub step: Option<f64>,
    /// Fine adjustment step amount (when holding Alt).
    pub fine_step: Option<f64>,
    /// Default initial value.
    pub default_value: serde_json::Value,
    /// Inspector grouping section (e.g. "Geometry", "Appearance", "Effects").
    pub group: String,
}

/// Registry storing all application property descriptors.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct PropertyRegistry {
    /// List of registered property descriptors.
    pub descriptors: Vec<PropertyDescriptor>,
}

impl PropertyRegistry {
    /// Initializes standard built-in property descriptors for all core shapes.
    pub fn standard() -> Self {
        let mut reg = Self::default();

        // Line properties
        reg.add(PropertyDescriptor {
            name: "line.length".into(),
            label: "Length".into(),
            tip: "How far each arm extends from its starting point in game pixels.".into(),
            unit: Some("px".into()),
            min: Some(0.0),
            max: Some(512.0),
            step: Some(0.5),
            fine_step: Some(0.1),
            default_value: serde_json::json!(6.0),
            group: "Geometry".into(),
        });

        reg.add(PropertyDescriptor {
            name: "line.thickness".into(),
            label: "Thickness".into(),
            tip: "Width of each crosshair line in game pixels.".into(),
            unit: Some("px".into()),
            min: Some(0.25),
            max: Some(64.0),
            step: Some(0.5),
            fine_step: Some(0.1),
            default_value: serde_json::json!(2.0),
            group: "Geometry".into(),
        });

        reg.add(PropertyDescriptor {
            name: "line.gap".into(),
            label: "Gap".into(),
            tip: "Inner distance between the center point and the start of each arm.".into(),
            unit: Some("px".into()),
            min: Some(0.0),
            max: Some(256.0),
            step: Some(0.5),
            fine_step: Some(0.1),
            default_value: serde_json::json!(4.0),
            group: "Geometry".into(),
        });

        reg.add(PropertyDescriptor {
            name: "line.arm_count".into(),
            label: "Arm Count".into(),
            tip: "Number of radially distributed arms around the center.".into(),
            unit: None,
            min: Some(1.0),
            max: Some(16.0),
            step: Some(1.0),
            fine_step: Some(1.0),
            default_value: serde_json::json!(4),
            group: "Geometry".into(),
        });

        // Dot properties
        reg.add(PropertyDescriptor {
            name: "dot.diameter".into(),
            label: "Center Dot Diameter".into(),
            tip: "Size of the central reticle dot in game pixels.".into(),
            unit: Some("px".into()),
            min: Some(0.5),
            max: Some(32.0),
            step: Some(0.5),
            fine_step: Some(0.1),
            default_value: serde_json::json!(2.0),
            group: "Geometry".into(),
        });

        // Glow properties
        reg.add(PropertyDescriptor {
            name: "glow.radius".into(),
            label: "Glow Radius".into(),
            tip: "Outer radiant dispersion radius for glow effects (max 64 px).".into(),
            unit: Some("px".into()),
            min: Some(0.0),
            max: Some(64.0),
            step: Some(0.5),
            fine_step: Some(0.1),
            default_value: serde_json::json!(4.0),
            group: "Effects".into(),
        });

        reg.add(PropertyDescriptor {
            name: "glow.intensity".into(),
            label: "Glow Intensity".into(),
            tip: "Brightness intensity multiplier of the glow bloom.".into(),
            unit: None,
            min: Some(0.0),
            max: Some(4.0),
            step: Some(0.1),
            fine_step: Some(0.05),
            default_value: serde_json::json!(1.0),
            group: "Effects".into(),
        });

        // Global transform
        reg.add(PropertyDescriptor {
            name: "global.scale".into(),
            label: "Master Scale".into(),
            tip: "Uniform scaling multiplier applied to the entire composite crosshair.".into(),
            unit: None,
            min: Some(0.1),
            max: Some(10.0),
            step: Some(0.05),
            fine_step: Some(0.01),
            default_value: serde_json::json!(1.0),
            group: "Transform".into(),
        });

        reg.add(PropertyDescriptor {
            name: "global.opacity".into(),
            label: "Master Opacity".into(),
            tip: "Overall visibility transparency of the reticle.".into(),
            unit: Some("%".into()),
            min: Some(0.0),
            max: Some(1.0),
            step: Some(0.05),
            fine_step: Some(0.01),
            default_value: serde_json::json!(1.0),
            group: "Appearance".into(),
        });

        reg
    }

    /// Registers a new property descriptor.
    pub fn add(&mut self, desc: PropertyDescriptor) {
        self.descriptors.push(desc);
    }

    /// Generates markdown documentation table for docs/PROPERTIES.md.
    pub fn to_markdown(&self) -> String {
        let mut md = String::from("# Crosshair Property Catalog\n\n");
        md.push_str("*Generated automatically from descriptor registry — single source of truth for UI, tooltips, and MCP.*\n\n");
        md.push_str(
            "| Property | Label | Group | Unit | Min..Max | Default | Tooltip Description |\n",
        );
        md.push_str("|---|---|---|---|---|---|---|\n");

        for d in &self.descriptors {
            let unit = d.unit.as_deref().unwrap_or("-");
            let range = match (d.min, d.max) {
                (Some(min), Some(max)) => format!("{min}..{max}"),
                _ => "-".to_string(),
            };
            md.push_str(&format!(
                "| `{}` | {} | {} | {} | {} | `{}` | {} |\n",
                d.name, d.label, d.group, unit, range, d.default_value, d.tip
            ));
        }

        md
    }
}
