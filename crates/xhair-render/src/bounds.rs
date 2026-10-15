//! Conservative bounding box calculation for crosshairs and individual layers.

use xhair_model::crosshair::Crosshair;
use xhair_model::layer::Layer;
use xhair_model::shapes::LayerKind;

/// Axis-aligned conservative bounding box `[min_x, min_y, max_x, max_y]`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    /// Minimum X coordinate in pixels.
    pub min_x: f32,
    /// Minimum Y coordinate in pixels.
    pub min_y: f32,
    /// Maximum X coordinate in pixels.
    pub max_x: f32,
    /// Maximum Y coordinate in pixels.
    pub max_y: f32,
}

impl Default for BoundingBox {
    fn default() -> Self {
        Self {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 0.0,
            max_y: 0.0,
        }
    }
}

impl BoundingBox {
    /// Creates a new bounding box.
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    /// Extends bounding box to include a given point.
    pub fn include_point(&mut self, x: f32, y: f32) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
    }

    /// Expands the bounding box outward by a given padding distance.
    pub fn expand(&mut self, padding: f32) {
        self.min_x -= padding;
        self.min_y -= padding;
        self.max_x += padding;
        self.max_y += padding;
    }

    /// Merges another bounding box into this one.
    pub fn union(&mut self, other: &BoundingBox) {
        self.min_x = self.min_x.min(other.min_x);
        self.min_y = self.min_y.min(other.min_y);
        self.max_x = self.max_x.max(other.max_x);
        self.max_y = self.max_y.max(other.max_y);
    }

    /// Computes total width.
    pub fn width(&self) -> f32 {
        (self.max_x - self.min_x).max(0.0)
    }

    /// Computes total height.
    pub fn height(&self) -> f32 {
        (self.max_y - self.min_y).max(0.0)
    }
}

/// Calculates conservative bounds for a single layer in local pixels.
pub fn calculate_layer_bounds(layer: &Layer) -> BoundingBox {
    let mut extent = match &layer.kind {
        LayerKind::Line(props) => (props.gap + props.length + props.thickness).max(4.0),
        LayerKind::Dot(props) => props.diameter * 0.5 + 2.0,
        LayerKind::Rect(props) => (props.width * 0.5).max(props.height * 0.5) + 2.0,
        LayerKind::Circle(props) => props.diameter * 0.5 + 2.0,
        LayerKind::Ring(props) => props.outer_radius + props.thickness + 2.0,
        LayerKind::Arc(props) => props.radius + props.thickness + 2.0,
        LayerKind::Polygon(props) => props.radius + 2.0,
        LayerKind::Triangle(props) => (props.base * 0.5).max(props.height * 0.5) + 2.0,
        LayerKind::Chevron(props) => props.gap + props.arm_length + props.thickness + 2.0,
        LayerKind::TShape(props) => {
            (props.bar_length * 0.5).max(props.gap + props.stem_length) + 2.0
        }
        LayerKind::Plus(props) => props.gap + props.length + props.thickness + 2.0,
        LayerKind::Pixels(props) => (props.width as f32 * 0.5).max(props.height as f32 * 0.5) + 2.0,
        LayerKind::Path(_) => 32.0,
        LayerKind::Image(_) | LayerKind::Group(_) => 32.0,
    };

    if let Some(ref stroke) = layer.stroke {
        extent += stroke.width;
    }

    if let Some(ref glow) = layer.effects.glow {
        extent += glow.radius.min(64.0);
    }

    if let Some(ref shadow) = layer.effects.shadow {
        extent += shadow.blur + shadow.offset.x.abs().max(shadow.offset.y.abs());
    }

    let scale_x = layer.transform.scale.x.abs().max(0.01);
    let scale_y = layer.transform.scale.y.abs().max(0.01);
    let max_scale = scale_x.max(scale_y);
    let scaled_extent = extent * max_scale;

    let ox = layer.transform.offset.x;
    let oy = layer.transform.offset.y;

    BoundingBox::new(
        ox - scaled_extent,
        oy - scaled_extent,
        ox + scaled_extent,
        oy + scaled_extent,
    )
}

/// Calculates the overall conservative bounding box for a crosshair document.
pub fn calculate_crosshair_bounds(crosshair: &Crosshair) -> BoundingBox {
    if crosshair.layers.is_empty() {
        return BoundingBox::new(-16.0, -16.0, 16.0, 16.0);
    }

    let mut total_bbox = BoundingBox::new(0.0, 0.0, 0.0, 0.0);
    for layer in &crosshair.layers {
        if !layer.visible {
            continue;
        }
        let layer_bbox = calculate_layer_bounds(layer);
        total_bbox.union(&layer_bbox);
    }

    // Include global transform and effects
    let global_scale = crosshair.global.scale.abs().max(0.01);
    total_bbox.min_x = (total_bbox.min_x * global_scale) + crosshair.global.offset.x;
    total_bbox.max_x = (total_bbox.max_x * global_scale) + crosshair.global.offset.x;
    total_bbox.min_y = (total_bbox.min_y * global_scale) + crosshair.global.offset.y;
    total_bbox.max_y = (total_bbox.max_y * global_scale) + crosshair.global.offset.y;

    if let Some(ref glow) = crosshair.global_effects.glow {
        total_bbox.expand(glow.radius.min(64.0));
    }
    if let Some(ref shadow) = crosshair.global_effects.shadow {
        total_bbox.expand(shadow.blur + shadow.offset.x.abs().max(shadow.offset.y.abs()));
    }

    // Minimum baseline margin of 2px
    total_bbox.expand(2.0);
    total_bbox
}
