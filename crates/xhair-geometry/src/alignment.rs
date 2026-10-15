//! Pixel grid alignment checks and automatic 1-click correction suggestions (§6.2).

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use xhair_model::crosshair::Crosshair;
use xhair_model::shapes::LayerKind;
use xhair_model::units::LayerId;

/// Axis along which an alignment issue was detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AlignmentAxis {
    /// Horizontal axis (X).
    Horizontal,
    /// Vertical axis (Y).
    Vertical,
    /// Both axes.
    Both,
}

/// Suggested correction to align an off-grid element.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SuggestedFix {
    /// Nudge position by 0.5 game pixels.
    OffsetHalfPixel {
        /// Suggested offset delta.
        delta: f32,
    },
    /// Adjust thickness or dimension by 1.0 game pixel.
    AdjustThickness {
        /// Suggested new thickness or size.
        new_thickness: f32,
    },
}

/// Detailed report of a geometry edge falling on a fractional sub-pixel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AlignmentIssue {
    /// Layer containing the misaligned geometry.
    pub layer_id: LayerId,
    /// Layer name.
    pub layer_name: String,
    /// Axis of misalignment.
    pub axis: AlignmentAxis,
    /// Current calculated edge coordinate.
    pub edge_position: f64,
    /// Actionable fix.
    pub suggested_fix: SuggestedFix,
}

/// Tests whether a symmetric stroke of thickness `t` centered on dimension `d` lands on integer pixel edges.
/// Formula: `(d + t)` must be an even integer.
pub fn is_symmetric_stroke_aligned(dimension: u32, thickness: f32) -> bool {
    let t_round = thickness.round();
    if (thickness - t_round).abs() > f32::EPSILON {
        return false;
    }
    (dimension + t_round as u32) % 2 == 0
}

/// Analyzes all layers in a crosshair against render resolution `(w, h)` and compiles alignment diagnostics.
pub fn alignment_report(crosshair: &Crosshair, width: u32, height: u32) -> Vec<AlignmentIssue> {
    let mut issues = Vec::new();

    for layer in &crosshair.layers {
        if !layer.visible {
            continue;
        }

        match &layer.kind {
            LayerKind::Line(line) => {
                let h_aligned = is_symmetric_stroke_aligned(width, line.thickness);
                let v_aligned = is_symmetric_stroke_aligned(height, line.thickness);

                if !h_aligned || !v_aligned {
                    let axis = if !h_aligned && !v_aligned {
                        AlignmentAxis::Both
                    } else if !h_aligned {
                        AlignmentAxis::Horizontal
                    } else {
                        AlignmentAxis::Vertical
                    };

                    issues.push(AlignmentIssue {
                        layer_id: layer.id,
                        layer_name: layer.name.clone(),
                        axis,
                        edge_position: (width as f64 / 2.0) - (line.thickness as f64 / 2.0),
                        suggested_fix: SuggestedFix::AdjustThickness {
                            new_thickness: (line.thickness + 1.0).max(1.0),
                        },
                    });
                }
            }
            LayerKind::Dot(dot) => {
                let h_aligned = is_symmetric_stroke_aligned(width, dot.diameter);
                let v_aligned = is_symmetric_stroke_aligned(height, dot.diameter);

                if !h_aligned || !v_aligned {
                    issues.push(AlignmentIssue {
                        layer_id: layer.id,
                        layer_name: layer.name.clone(),
                        axis: AlignmentAxis::Both,
                        edge_position: (width as f64 / 2.0) - (dot.diameter as f64 / 2.0),
                        suggested_fix: SuggestedFix::AdjustThickness {
                            new_thickness: (dot.diameter + 1.0).max(1.0),
                        },
                    });
                }
            }
            LayerKind::Rect(rect) => {
                let h_aligned = is_symmetric_stroke_aligned(width, rect.width);
                let v_aligned = is_symmetric_stroke_aligned(height, rect.height);

                if !h_aligned || !v_aligned {
                    let axis = if !h_aligned && !v_aligned {
                        AlignmentAxis::Both
                    } else if !h_aligned {
                        AlignmentAxis::Horizontal
                    } else {
                        AlignmentAxis::Vertical
                    };

                    let new_dim = if !h_aligned {
                        (rect.width + 1.0).max(1.0)
                    } else {
                        (rect.height + 1.0).max(1.0)
                    };

                    issues.push(AlignmentIssue {
                        layer_id: layer.id,
                        layer_name: layer.name.clone(),
                        axis,
                        edge_position: (width as f64 / 2.0) - (rect.width as f64 / 2.0),
                        suggested_fix: SuggestedFix::AdjustThickness {
                            new_thickness: new_dim,
                        },
                    });
                }
            }
            LayerKind::Circle(circle) => {
                let h_aligned = is_symmetric_stroke_aligned(width, circle.diameter);
                let v_aligned = is_symmetric_stroke_aligned(height, circle.diameter);

                if !h_aligned || !v_aligned {
                    issues.push(AlignmentIssue {
                        layer_id: layer.id,
                        layer_name: layer.name.clone(),
                        axis: AlignmentAxis::Both,
                        edge_position: (width as f64 / 2.0) - (circle.diameter as f64 / 2.0),
                        suggested_fix: SuggestedFix::AdjustThickness {
                            new_thickness: (circle.diameter + 1.0).max(1.0),
                        },
                    });
                }
            }
            _ => {}
        }
    }

    issues
}

/// Applies an alignment suggestion to the target layer in the crosshair (1-click automatic fix).
pub fn apply_suggested_fix(crosshair: &mut Crosshair, issue: &AlignmentIssue) -> bool {
    let layer = match crosshair.layers.iter_mut().find(|l| l.id == issue.layer_id) {
        Some(l) => l,
        None => return false,
    };

    match &issue.suggested_fix {
        SuggestedFix::AdjustThickness { new_thickness } => match &mut layer.kind {
            LayerKind::Line(l) => {
                l.thickness = *new_thickness;
                true
            }
            LayerKind::Dot(d) => {
                d.diameter = *new_thickness;
                true
            }
            LayerKind::Circle(c) => {
                c.diameter = *new_thickness;
                true
            }
            LayerKind::Rect(r) => {
                if issue.axis == AlignmentAxis::Horizontal || issue.axis == AlignmentAxis::Both {
                    r.width = *new_thickness;
                }
                if issue.axis == AlignmentAxis::Vertical || issue.axis == AlignmentAxis::Both {
                    r.height = *new_thickness;
                }
                true
            }
            _ => false,
        },
        SuggestedFix::OffsetHalfPixel { delta } => {
            match issue.axis {
                AlignmentAxis::Horizontal => layer.transform.offset.x += delta,
                AlignmentAxis::Vertical => layer.transform.offset.y += delta,
                AlignmentAxis::Both => {
                    layer.transform.offset.x += delta;
                    layer.transform.offset.y += delta;
                }
            }
            true
        }
    }
}
