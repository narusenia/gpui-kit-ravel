//! JSON schemas for the gpui types that carry no `JsonSchema` impl.
//!
//! `Hsla` is a `palette` alias, and `BoxShadow` derives nothing, so schemars
//! cannot describe either on its own. gpui exports `gpui::hsla_schemar` for a
//! bare `Hsla`; the wrappers here cover the `Option` and `Vec` shapes this
//! crate stores.

use schemars::{Schema, SchemaGenerator, json_schema};

/// Schema for `Option<Hsla>`.
pub(crate) fn option_hsla_schemar(generator: &mut SchemaGenerator) -> Schema {
    nullable(gpui::hsla_schemar(generator))
}

/// Schema for `Vec<BoxShadow>`.
pub(crate) fn box_shadows_schemar(generator: &mut SchemaGenerator) -> Schema {
    let color = gpui::hsla_schemar(generator);
    let pixels = json_schema!({ "type": "number", "format": "float" });
    json_schema!({
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "color": color,
                "offset": {
                    "type": "object",
                    "properties": { "x": pixels, "y": pixels },
                },
                "blur_radius": pixels,
                "spread_radius": pixels,
                "inset": { "type": "boolean" },
            },
        },
    })
}

/// Schema for `Option<Vec<BoxShadow>>`.
pub(crate) fn option_box_shadows_schemar(generator: &mut SchemaGenerator) -> Schema {
    nullable(box_shadows_schemar(generator))
}

/// Wrap a schema so `null` validates against it as well.
///
/// A `schema_with` attribute replaces the whole schema schemars would have
/// derived, including the nullability it adds for an `Option` field, so the
/// wrappers above have to put it back.
fn nullable(inner: Schema) -> Schema {
    json_schema!({ "anyOf": [inner, { "type": "null" }] })
}
