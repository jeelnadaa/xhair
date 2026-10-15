//! Shape primitive rasterizers.

pub mod circle;
pub mod dot;
pub mod lines;
pub mod path;
pub mod pixels;
pub mod plus;
pub mod polygon;
pub mod rect;
pub mod t_shape;

use tiny_skia::{Pixmap, Transform};
use xhair_model::layer::{ArmOverride, Layer};
use xhair_model::shapes::LayerKind;

/// Dispatches rasterization of a layer's shape primitive.
pub fn render_shape(
    layer: &Layer,
    arm_override: Option<&ArmOverride>,
    instance_transform: Transform,
    opacity: f32,
    anti_alias: bool,
    pixmap: &mut Pixmap,
) {
    match &layer.kind {
        LayerKind::Line(props) => {
            lines::render_lines(
                props,
                layer,
                arm_override,
                instance_transform,
                opacity,
                anti_alias,
                pixmap,
            );
        }
        LayerKind::Dot(props) => {
            dot::render_dot(
                props,
                layer,
                instance_transform,
                opacity,
                anti_alias,
                pixmap,
            );
        }
        LayerKind::Rect(props) => {
            rect::render_rect(
                props,
                layer,
                instance_transform,
                opacity,
                anti_alias,
                pixmap,
            );
        }
        LayerKind::Circle(props) => {
            circle::render_circle(
                props,
                layer,
                instance_transform,
                opacity,
                anti_alias,
                pixmap,
            );
        }
        LayerKind::Ring(props) => {
            circle::render_ring(
                props,
                layer,
                instance_transform,
                opacity,
                anti_alias,
                pixmap,
            );
        }
        LayerKind::Arc(props) => {
            circle::render_arc(
                props,
                layer,
                instance_transform,
                opacity,
                anti_alias,
                pixmap,
            );
        }
        LayerKind::Polygon(props) => {
            polygon::render_polygon(
                props,
                layer,
                instance_transform,
                opacity,
                anti_alias,
                pixmap,
            );
        }
        LayerKind::Triangle(props) => {
            polygon::render_triangle(
                props,
                layer,
                instance_transform,
                opacity,
                anti_alias,
                pixmap,
            );
        }
        LayerKind::Chevron(props) => {
            polygon::render_chevron(
                props,
                layer,
                instance_transform,
                opacity,
                anti_alias,
                pixmap,
            );
        }
        LayerKind::TShape(props) => {
            t_shape::render_t_shape(
                props,
                layer,
                instance_transform,
                opacity,
                anti_alias,
                pixmap,
            );
        }
        LayerKind::Plus(props) => {
            plus::render_plus(
                props,
                layer,
                instance_transform,
                opacity,
                anti_alias,
                pixmap,
            );
        }
        LayerKind::Path(props) => {
            path::render_path(
                props,
                layer,
                instance_transform,
                opacity,
                anti_alias,
                pixmap,
            );
        }
        LayerKind::Pixels(props) => {
            pixels::render_pixels(props, instance_transform, opacity, pixmap);
        }
        LayerKind::Image(_) | LayerKind::Group(_) => {
            // Handled or container
        }
    }
}
