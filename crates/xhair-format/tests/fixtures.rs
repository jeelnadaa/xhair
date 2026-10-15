use xhair_format::migrations::migrate_document_json;
use xhair_format::FormatError;
use xhair_model::crosshair::{
    Crosshair, CURRENT_FORMAT_VERSION, MAX_ASSETS_BYTES, MAX_LAYERS, MAX_NAME_LENGTH,
    MAX_NESTING_DEPTH,
};
use xhair_model::layer::Layer;
use xhair_model::shapes::{GroupProps, LayerKind};
use xhair_model::units::LayerId;

#[test]
fn test_v1_classic_fixture_loads_and_validates() {
    let content = include_str!("fixtures/formats/v1_classic.xhair");
    let json_val: serde_json::Value = serde_json::from_str(content).expect("Valid JSON");
    let migrated = migrate_document_json(json_val).expect("Migration succeeds");
    let crosshair: Crosshair = serde_json::from_value(migrated).expect("Deserialization succeeds");

    assert_eq!(crosshair.format_version, CURRENT_FORMAT_VERSION);
    assert_eq!(crosshair.name, "V1 Classic Test");
    assert_eq!(crosshair.layers.len(), 1);
    assert!(crosshair.validate().is_ok());
}

#[test]
fn test_future_version_rejected() {
    let content = include_str!("fixtures/formats/v999_future.xhair");
    let json_val: serde_json::Value = serde_json::from_str(content).expect("Valid JSON");
    let err = migrate_document_json(json_val).unwrap_err();

    match err {
        FormatError::UnsupportedVersion {
            version,
            max_supported,
        } => {
            assert_eq!(version, 999);
            assert_eq!(max_supported, CURRENT_FORMAT_VERSION);
        }
        other => panic!("Expected UnsupportedVersion, got {other:?}"),
    }
}

#[test]
fn test_oversized_name_rejected() {
    let long_name = "A".repeat(MAX_NAME_LENGTH + 1);
    let xhair = Crosshair::new_blank(long_name);
    assert!(xhair.validate().is_err());
}

#[test]
fn test_oversized_layers_rejected() {
    let mut xhair = Crosshair::new_blank("Oversized Layers");
    for i in 0..=(MAX_LAYERS) {
        xhair.layers.push(Layer::new(
            format!("Layer {i}"),
            LayerKind::Dot(Default::default()),
        ));
    }
    assert!(xhair.validate().is_err());
}

#[test]
fn test_oversized_assets_rejected() {
    let mut xhair = Crosshair::new_blank("Oversized Assets");
    let large_asset = xhair_model::crosshair::Asset {
        hash: "dummy".to_string(),
        mime: "image/png".to_string(),
        data_base64: "A".repeat(MAX_ASSETS_BYTES + 10),
    };
    xhair.assets.insert("large".to_string(), large_asset);
    assert!(xhair.validate().is_err());
}

#[test]
fn test_group_nesting_limit() {
    let mut xhair = Crosshair::new_blank("Deep Groups");

    let mut child_id = LayerId::new_v7();
    let mut leaf = Layer::new("Leaf", LayerKind::Dot(Default::default()));
    leaf.id = child_id;
    xhair.layers.push(leaf);

    for i in 0..=(MAX_NESTING_DEPTH + 1) {
        let parent_id = LayerId::new_v7();
        let mut group_layer = Layer::new(
            format!("Group {i}"),
            LayerKind::Group(GroupProps {
                children: vec![child_id],
            }),
        );
        group_layer.id = parent_id;
        xhair.layers.push(group_layer);
        child_id = parent_id;
    }

    assert!(xhair.validate().is_err());
}

#[test]
fn test_group_cycle_detected() {
    let mut xhair = Crosshair::new_blank("Cycle Groups");
    let id_a = LayerId::new_v7();
    let id_b = LayerId::new_v7();

    let mut group_a = Layer::new(
        "Group A",
        LayerKind::Group(GroupProps {
            children: vec![id_b],
        }),
    );
    group_a.id = id_a;

    let mut group_b = Layer::new(
        "Group B",
        LayerKind::Group(GroupProps {
            children: vec![id_a],
        }),
    );
    group_b.id = id_b;

    xhair.layers.push(group_a);
    xhair.layers.push(group_b);

    let res = xhair.validate();
    assert!(res.is_err());
    assert!(res.unwrap_err().to_string().contains("Cycle detected"));
}
