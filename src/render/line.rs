use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

lazy_static! {
    static ref ROUNDED_STROKE_OPTIONS: StrokeOptions = {
        let mut out: StrokeOptions = default();
        out.start_cap = LineCap::Round;
        out.end_cap = LineCap::Round;
        out.line_join = LineJoin::Round;
        return out;
    };
}

pub fn rounded_stroke(line_width: f32, color: Color) -> StrokeMode {
    StrokeMode {
        options: ROUNDED_STROKE_OPTIONS.with_line_width(line_width),
        color,
    }
}
