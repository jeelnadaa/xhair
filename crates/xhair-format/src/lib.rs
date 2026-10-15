//! File format encoders, decoders, Brotli share codes, bundles, and migrations for xhair.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod bundle;
pub mod canonical;
pub mod error;
pub mod migrations;
pub mod share_code;
pub mod valorant;

pub use bundle::{export_pack, import_pack, PackEntry, PackManifest, PACK_FORMAT_VERSION};
pub use canonical::{canonicalize_value, to_canonical_json};
pub use error::FormatError;
pub use migrations::migrate_document_json;
pub use share_code::{decode_share_code, encode_share_code, MAX_SHARE_CODE_LEN, SHARE_CODE_PREFIX};
pub use valorant::{import_valorant_code, ValorantImportResult};

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use xhair_model::crosshair::Crosshair;

    #[test]
    fn test_canonical_json_determinism() {
        let xhair = Crosshair::new_classic("Deterministic Reticle");
        let json1 = to_canonical_json(&xhair).unwrap();
        let json2 = to_canonical_json(&xhair).unwrap();
        assert_eq!(json1, json2);
        // Verify no extra spaces
        assert!(!json1.contains(": "));
        assert!(!json1.contains(", "));
    }

    #[test]
    fn test_share_code_round_trip() {
        let original = Crosshair::new_classic("Share Code Test");
        let code = encode_share_code(&original).expect("Encoding share code failed");

        assert!(code.starts_with("XH1-"));
        println!("Generated share code (len {}): {}", code.len(), code);

        let decoded = decode_share_code(&code).expect("Decoding share code failed");
        assert_eq!(original.id, decoded.id);
        assert_eq!(original.name, decoded.name);
        assert_eq!(original.layers.len(), decoded.layers.len());
    }

    #[test]
    fn test_share_code_checksum_corruption_detection() {
        let original = Crosshair::new_classic("CRC Test");
        let code = encode_share_code(&original).unwrap();

        // Mutate one character of the checksum
        let mut corrupted = code.into_bytes();
        let last_idx = corrupted.len() - 1;
        corrupted[last_idx] = if corrupted[last_idx] == b'a' {
            b'b'
        } else {
            b'a'
        };
        let corrupted_code = String::from_utf8(corrupted).unwrap();

        let err = decode_share_code(&corrupted_code).unwrap_err();
        match err {
            FormatError::CorruptedCrc { .. } => {}
            other => panic!("Expected CorruptedCrc, got {other:?}"),
        }
    }

    #[test]
    fn test_bundle_pack_round_trip() {
        let xhair1 = Crosshair::new_classic("Pack Reticle 1");
        let xhair2 = Crosshair::new_blank("Pack Reticle 2");
        let list = vec![xhair1.clone(), xhair2.clone()];

        let pack_bytes = export_pack(&list, "Test Bundle", "Collection", "Author").unwrap();
        assert!(!pack_bytes.is_empty());

        let (manifest, imported) = import_pack(&pack_bytes).unwrap();
        assert_eq!(manifest.title, "Test Bundle");
        assert_eq!(imported.len(), 2);
        assert_eq!(imported[0].name, xhair1.name);
        assert_eq!(imported[1].name, xhair2.name);
    }

    #[test]
    fn test_valorant_code_importer() {
        // Classic 1-4-2-0 green crosshair from Valorant
        let code = "0;P;c;1;h;0;0l;4;0o;2;0a;1;0f;0;1b;0";
        let res = import_valorant_code(code).expect("Import failed");

        assert_eq!(res.crosshair.layers.len(), 1);
        let layer = &res.crosshair.layers[0];
        assert_eq!(layer.name, "Inner Lines");
        if let xhair_model::shapes::LayerKind::Line(ref p) = layer.kind {
            assert_eq!(p.length, 4.0);
            assert_eq!(p.gap, 2.0);
        } else {
            panic!("Expected line layer");
        }
    }

    #[test]
    fn test_valorant_code_with_dot_and_outer_lines() {
        // Dot + inner + outer lines
        let code = "0;P;c;5;h;1;t;1;o;0.8;d;1;z;2;0t;1;0l;3;0o;1;1b;1;1t;1;1l;2;1o;8";
        let res = import_valorant_code(code).expect("Import failed");

        assert_eq!(res.crosshair.layers.len(), 3);
        assert_eq!(res.crosshair.layers[0].name, "Center Dot");
        assert_eq!(res.crosshair.layers[1].name, "Inner Lines");
        assert_eq!(res.crosshair.layers[2].name, "Outer Lines");
    }

    proptest! {
        #[test]
        fn prop_share_code_round_trip(length in 1.0f32..32.0f32, gap in 0.0f32..16.0f32, thickness in 0.5f32..8.0f32) {
            let mut xhair = Crosshair::new_classic("Prop Test");
            if let xhair_model::shapes::LayerKind::Line(ref mut p) = xhair.layers[0].kind {
                p.length = length;
                p.gap = gap;
                p.thickness = thickness;
            }

            let code = encode_share_code(&xhair).unwrap();
            let decoded = decode_share_code(&code).unwrap();

            if let xhair_model::shapes::LayerKind::Line(ref p) = decoded.layers[0].kind {
                prop_assert!((p.length - length).abs() < 0.01);
                prop_assert!((p.gap - gap).abs() < 0.01);
                prop_assert!((p.thickness - thickness).abs() < 0.01);
            }
        }
    }
}
