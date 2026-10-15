//! Core domain models, schema derivation, validation, and error types for xhair.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod crosshair;
pub mod effects;
pub mod error;
pub mod layer;
pub mod paint;
pub mod settings;
pub mod shapes;
pub mod units;

pub use crosshair::*;
pub use effects::*;
pub use error::XhairError;
pub use layer::*;
pub use paint::*;
pub use settings::*;
pub use shapes::*;
pub use units::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crosshair_blank_creation_and_validation() {
        let xhair = Crosshair::new_blank("Test Dot");
        assert_eq!(xhair.name, "Test Dot");
        assert_eq!(xhair.format_version, CURRENT_FORMAT_VERSION);
        assert!(xhair.validate().is_ok());
    }

    #[test]
    fn test_crosshair_classic_template() {
        let xhair = Crosshair::new_classic("Classic 4-Arm");
        assert_eq!(xhair.layers.len(), 1);
        assert!(xhair.validate().is_ok());
    }

    #[test]
    fn test_crosshair_name_validation() {
        let mut xhair = Crosshair::new_blank("");
        assert!(matches!(xhair.validate(), Err(XhairError::Validation(_))));

        xhair.name = "A".repeat(MAX_NAME_LENGTH + 1);
        assert!(matches!(xhair.validate(), Err(XhairError::Validation(_))));
    }

    #[test]
    fn test_layer_polygon_validation() {
        let mut xhair = Crosshair::new_blank("Invalid Poly");
        xhair.layers.push(Layer::new(
            "Poly",
            LayerKind::Polygon(PolygonProps {
                sides: 2, // Must be >= 3
                ..Default::default()
            }),
        ));
        assert!(matches!(xhair.validate(), Err(XhairError::Validation(_))));
    }

    #[test]
    fn test_serialization_round_trip() {
        let xhair = Crosshair::new_classic("Round Trip Reticle");
        let json = serde_json::to_string_pretty(&xhair).expect("Serialization failed");
        let deserialized: Crosshair = serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(xhair.id, deserialized.id);
        assert_eq!(xhair.name, deserialized.name);
        assert_eq!(xhair.layers.len(), deserialized.layers.len());
    }

    #[test]
    fn test_error_code_mapping() {
        let err = XhairError::Validation("Invalid property".to_string());
        assert_eq!(err.code(), "XH-1001");
    }
}
