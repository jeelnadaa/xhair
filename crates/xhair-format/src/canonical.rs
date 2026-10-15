//! Canonical JSON serialization with deterministic key sorting and normalized float precision.

use crate::error::FormatError;
use serde_json::{Map, Number, Value};
use xhair_model::crosshair::Crosshair;

/// Normalizes a `serde_json::Value` by sorting object keys alphabetically and clamping float precision to 4 decimals.
pub fn canonicalize_value(val: Value) -> Value {
    match val {
        Value::Object(map) => {
            let mut entries: Vec<(String, Value)> = map
                .into_iter()
                .map(|(k, v)| (k, canonicalize_value(v)))
                .collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));

            let mut sorted_map = Map::with_capacity(entries.len());
            for (k, v) in entries {
                sorted_map.insert(k, v);
            }
            Value::Object(sorted_map)
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(canonicalize_value).collect()),
        Value::Number(num) => {
            if let Some(f) = num.as_f64() {
                // Round float to 4 decimal places
                let rounded = (f * 10_000.0).round() / 10_000.0;
                // If the number is an integer, keep integer representation
                if (rounded - rounded.round()).abs() < f64::EPSILON
                    && rounded >= i64::MIN as f64
                    && rounded <= i64::MAX as f64
                {
                    Value::Number(Number::from(rounded as i64))
                } else if let Some(n) = Number::from_f64(rounded) {
                    Value::Number(n)
                } else {
                    Value::Number(num)
                }
            } else {
                Value::Number(num)
            }
        }
        other => other,
    }
}

/// Serializes a Crosshair into canonical, deterministic, whitespace-free JSON bytes.
pub fn to_canonical_json(crosshair: &Crosshair) -> Result<String, FormatError> {
    let raw_value =
        serde_json::to_value(crosshair).map_err(|e| FormatError::Serialization(e.to_string()))?;

    let canonical_value = canonicalize_value(raw_value);

    serde_json::to_string(&canonical_value).map_err(|e| FormatError::Serialization(e.to_string()))
}
