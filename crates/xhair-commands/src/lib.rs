//! Command bus, undo/redo, command registry, and property descriptors for xhair.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod actor;
pub mod bus;
pub mod command;
pub mod descriptors;
pub mod diff;
pub mod history;

pub use actor::*;
pub use bus::*;
pub use command::*;
pub use descriptors::*;
pub use diff::*;
pub use history::*;

#[cfg(test)]
mod tests {
    use super::*;
    use xhair_model::crosshair::Crosshair;
    use xhair_model::layer::Layer;
    use xhair_model::shapes::{LayerKind, LineProps};

    #[test]
    fn test_command_dispatch_and_undo() {
        let mut bus = CommandBus::default();
        let xhair = Crosshair::new_blank("Test Doc");
        let cid = xhair.id;
        bus.register_document(xhair);

        let layer = Layer::new("Arms", LayerKind::Line(LineProps::default()));
        let lid = layer.id;

        // Add Layer
        let diff = bus
            .dispatch(
                Command::LayerAdd {
                    crosshair_id: cid,
                    layer,
                },
                DispatchOptions::default(),
            )
            .expect("Add layer failed");

        assert_eq!(diff.patches.len(), 1);
        assert_eq!(bus.documents.get(&cid).unwrap().layers.len(), 1);

        // Undo
        let undo_diff = bus.undo(cid).expect("Undo failed");
        assert!(undo_diff.is_some());
        assert_eq!(bus.documents.get(&cid).unwrap().layers.len(), 0);

        // Redo
        let redo_diff = bus.redo(cid).expect("Redo failed");
        assert!(redo_diff.is_some());
        assert_eq!(bus.documents.get(&cid).unwrap().layers.len(), 1);
        assert_eq!(bus.documents.get(&cid).unwrap().layers[0].id, lid);
    }

    #[test]
    fn test_dry_run_does_not_mutate_state() {
        let mut bus = CommandBus::default();
        let xhair = Crosshair::new_blank("Dry Run Doc");
        let cid = xhair.id;
        bus.register_document(xhair);

        let layer = Layer::new("Sample", LayerKind::Line(LineProps::default()));
        let diff = bus
            .dispatch(
                Command::LayerAdd {
                    crosshair_id: cid,
                    layer,
                },
                DispatchOptions {
                    dry_run: true,
                    ..Default::default()
                },
            )
            .expect("Dry run failed");

        assert_eq!(diff.patches.len(), 1);
        // State remains unchanged!
        assert_eq!(bus.documents.get(&cid).unwrap().layers.len(), 0);
        assert!(!bus.histories.get(&cid).unwrap().can_undo());
    }

    #[test]
    fn test_batch_atomic_execution_and_single_undo() {
        let mut bus = CommandBus::default();
        let xhair = Crosshair::new_blank("Batch Doc");
        let cid = xhair.id;
        bus.register_document(xhair);

        let l1 = Layer::new("L1", LayerKind::Line(LineProps::default()));
        let l2 = Layer::new("L2", LayerKind::Line(LineProps::default()));

        let batch_cmd = Command::Batch {
            commands: vec![
                Command::LayerAdd {
                    crosshair_id: cid,
                    layer: l1,
                },
                Command::LayerAdd {
                    crosshair_id: cid,
                    layer: l2,
                },
            ],
        };

        let diff = bus
            .dispatch(batch_cmd, DispatchOptions::default())
            .expect("Batch dispatch failed");
        assert_eq!(diff.patches.len(), 2);
        assert_eq!(bus.documents.get(&cid).unwrap().layers.len(), 2);

        // A single undo reverses both commands in the batch
        bus.undo(cid).expect("Batch undo failed");
        assert_eq!(bus.documents.get(&cid).unwrap().layers.len(), 0);
    }

    #[test]
    fn test_coalescing_burst_nudges() {
        let mut bus = CommandBus::default();
        let xhair = Crosshair::new_blank("Coalesce Doc");
        let cid = xhair.id;
        bus.register_document(xhair);

        // Rapid nudge sequence within 400ms window
        for i in 1..=5 {
            bus.dispatch(
                Command::Nudge {
                    crosshair_id: cid,
                    target: NudgeTarget::WholeCrosshair,
                    dx: 1.0,
                    dy: 0.0,
                    frame: NudgeFrame::Screen,
                },
                DispatchOptions {
                    timestamp_ms: Some(1000 + i * 50), // 50ms intervals
                    ..Default::default()
                },
            )
            .expect("Nudge failed");
        }

        assert_eq!(bus.documents.get(&cid).unwrap().global.offset.x, 5.0);

        // Because of coalescing, undo stack has only 1 entry!
        assert_eq!(bus.histories.get(&cid).unwrap().undo_stack.len(), 1);

        // A single undo resets the entire 5.0 offset back to 0.0
        bus.undo(cid).expect("Undo failed");
        assert_eq!(bus.documents.get(&cid).unwrap().global.offset.x, 0.0);
    }

    #[test]
    fn test_property_descriptors_registry_and_markdown() {
        let registry = PropertyRegistry::standard();
        assert!(!registry.descriptors.is_empty());

        let md = registry.to_markdown();
        assert!(md.contains("line.length"));
        assert!(md.contains("line.thickness"));
        assert!(md.contains("glow.radius"));

        // Keep ui/src/generated/properties.json in sync
        let json_str = serde_json::to_string_pretty(&registry).unwrap();
        let path = std::path::Path::new("../../ui/src/generated/properties.json");
        if let Ok(mut f) = std::fs::File::create(path) {
            use std::io::Write;
            let _ = f.write_all(json_str.as_bytes());
        }
    }

    #[test]
    fn test_v2_commands_dispatch_and_undo() {
        let mut bus = CommandBus::default();
        let xhair = Crosshair::new_blank("Old Name");
        let cid = xhair.id;
        bus.register_document(xhair);

        // 1. Rename document
        bus.dispatch(
            Command::CrosshairRename {
                crosshair_id: cid,
                new_name: "New Name".to_string(),
            },
            DispatchOptions::default(),
        )
        .expect("Rename doc failed");
        assert_eq!(bus.documents.get(&cid).unwrap().name, "New Name");

        // 2. Add layer at (0, 0)
        let layer = Layer::new("Arm 1", LayerKind::Line(LineProps::default()));
        let lid = layer.id;
        bus.dispatch(
            Command::LayerAdd {
                crosshair_id: cid,
                layer,
            },
            DispatchOptions::default(),
        )
        .expect("Add layer failed");

        // 3. Set position
        bus.dispatch(
            Command::LayerSetPosition {
                crosshair_id: cid,
                ids: vec![lid],
                x: 10.0,
                y: -5.0,
                mode: PositionMode::Absolute,
            },
            DispatchOptions::default(),
        )
        .expect("Set position failed");
        assert_eq!(
            bus.documents.get(&cid).unwrap().layers[0]
                .transform
                .offset
                .x,
            10.0
        );
        assert_eq!(
            bus.documents.get(&cid).unwrap().layers[0]
                .transform
                .offset
                .y,
            -5.0
        );

        // 4. Recenter layer
        bus.dispatch(
            Command::LayerRecenter {
                crosshair_id: cid,
                ids: vec![lid],
            },
            DispatchOptions::default(),
        )
        .expect("Recenter layer failed");
        assert_eq!(
            bus.documents.get(&cid).unwrap().layers[0]
                .transform
                .offset
                .x,
            0.0
        );
        assert_eq!(
            bus.documents.get(&cid).unwrap().layers[0]
                .transform
                .offset
                .y,
            0.0
        );

        // 5. Paint set
        bus.dispatch(
            Command::PaintSet {
                crosshair_id: cid,
                ids: vec![lid],
                color: "#FF0000FF".to_string(),
            },
            DispatchOptions::default(),
        )
        .expect("Paint set failed");

        // 6. Layer group
        bus.dispatch(
            Command::LayerGroup {
                crosshair_id: cid,
                ids: vec![lid],
                name: Some("My Group".to_string()),
            },
            DispatchOptions::default(),
        )
        .expect("Layer group failed");
        assert_eq!(bus.documents.get(&cid).unwrap().layers.len(), 2);

        // 7. Undo group
        bus.undo(cid).expect("Undo group failed");
        assert_eq!(bus.documents.get(&cid).unwrap().layers.len(), 1);
    }
}
