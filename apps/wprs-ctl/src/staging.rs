use crate::sowed::pair;
use crate::staged::{indexed, mode, takes};
use crate::steps::value;
use clap::Subcommand;
use serde_json::{Map, Value, json};
use wprs_editor::actions::LayerEdit as L;

const XY: &str = "X,Y";

#[derive(Subcommand, Debug)]
pub enum Stage {
    #[command(about = "Place, turn, scale, skew, tilt, lift or size the layer; what is not given stays")]
    Transform {
        #[arg(long, value_name = XY, value_parser = pair, allow_hyphen_values = true, help = "Pixels of the canvas")]
        position: Option<[f64; 2]>,
        #[arg(long, allow_hyphen_values = true, help = "Degrees from -360 to 360")]
        rotation: Option<f64>,
        #[arg(long, value_name = XY, value_parser = pair, allow_hyphen_values = true, help = "1,1 is the size it has")]
        scale: Option<[f64; 2]>,
        #[arg(long, value_name = XY, value_parser = pair, allow_hyphen_values = true, help = "Degrees")]
        skew: Option<[f64; 2]>,
        #[arg(long, value_name = XY, value_parser = pair, allow_hyphen_values = true, help = "Point it turns and scales around, in fractions of the layer")]
        origin: Option<[f64; 2]>,
        #[arg(long, value_name = XY, value_parser = pair, allow_hyphen_values = true, help = "Degrees from -89 to 89 the layer leans in depth around its position: X turns the bottom edge towards the viewer, Y the left edge")]
        tilt: Option<[f64; 2]>,
        #[arg(long, allow_hyphen_values = true, help = "Pixels towards the viewer, from -100000 to 100000")]
        z: Option<f64>,
        #[arg(allow_hyphen_values = true, long, value_name = "W,H", value_parser = pair, help = "Pixels of an image, video, solid, adjustment or spectrum layer")]
        size: Option<[f64; 2]>,
        #[arg(
            long,
            value_name = "S",
            conflicts_with = "replace",
            help = "Second of the scene at which a value with keyframes, or a part keyed on its own, takes a keyframe instead of being refused"
        )]
        at: Option<f64>,
        #[arg(long, help = crate::sowing::REPLACE)]
        replace: bool,
    },
    #[command(
        about = "Drag a handle of the canvas to a point of the canvas: move, nudge (by X,Y), turn, origin, scale or skew"
    )]
    Grip {
        #[arg(value_parser = ["move", "nudge", "turn", "origin", "scale", "skew"], help = "The handle")]
        grip: String,
        #[arg(value_name = XY, value_parser = pair, allow_hyphen_values = true, help = "Pixels of the canvas the handle goes to, or for nudge how far it moves")]
        to: [f64; 2],
        #[arg(long, value_name = XY, value_parser = pair, default_value = "1,1", help = "Point of the box a scale or skew holds, in fractions: 1,1 the bottom right corner, 1,0.5 the middle of the right edge")]
        corner: [f64; 2],
        #[arg(long, help = "Keep the proportions while scaling")]
        even: bool,
        #[arg(
            long,
            value_name = "PX",
            help = "Snap a move to the canvas and the other layers within these pixels"
        )]
        snap: Option<f64>,
        #[arg(long, value_name = "S", help = "Second of the playhead")]
        at: Option<f64>,
        #[arg(long, help = crate::sowing::REPLACE)]
        replace: bool,
    },
    #[command(about = "Scale and place the layer so that it covers the canvas, fits inside it or fills it")]
    Fit {
        #[arg(value_parser = ["cover", "contain", "stretch"], help = "cover, contain or stretch")]
        fit: String,
        #[arg(long, value_name = "S", help = "Second of the playhead")]
        at: Option<f64>,
        #[arg(long, help = crate::sowing::REPLACE)]
        replace: bool,
    },
    #[command(
        about = "Make the layer follow another one, or none without PARENT; --keep leaves it where it is"
    )]
    Parent {
        #[arg(help = "The layer to follow")]
        parent: Option<String>,
        #[arg(long, help = "Write the transform that keeps the layer where it is")]
        keep: bool,
        #[arg(long, value_name = "S", help = "Second of the playhead")]
        at: Option<f64>,
        #[arg(long, help = crate::sowing::REPLACE)]
        replace: bool,
    },
    #[command(
        about = "Blend the layer over what is below it with a mode such as normal, add, multiply or screen"
    )]
    Blend {
        #[arg(help = "A blend mode such as normal, add, multiply or screen", value_parser = mode)]
        blend: wprs_scene::Blend,
    },
    #[command(
        about = "Let the pointer through the layer (off), take it over the whole box (all) or only where the layer is at least alpha opaque (alpha)"
    )]
    Touch {
        #[arg(help = "off, all or alpha", value_parser = takes)]
        takes: wprs_editor::actions::Takes,
        #[arg(long, help = "From 0 to 1; with alpha, 0.5 when it had none")]
        alpha: Option<f64>,
    },
    #[command(
        about = "Change one particles layer from the system it draws: alpha, size, count, speed, life, time scale, brightness, color and control points"
    )]
    Instance {
        #[arg(allow_negative_numbers = true, long, help = "From 0 to 1")]
        alpha: Option<f64>,
        #[arg(allow_negative_numbers = true, long, help = "Times the size, from 0 to 100")]
        size: Option<f64>,
        #[arg(allow_negative_numbers = true, long, help = "Share of the particles, from 0 to 1")]
        count: Option<f64>,
        #[arg(allow_negative_numbers = true, long, help = "Times the speed, from 0 to 100")]
        speed: Option<f64>,
        #[arg(allow_negative_numbers = true, long, help = "Times the life, from 0 to 100")]
        life: Option<f64>,
        #[arg(allow_negative_numbers = true, long, help = "Times the pace of time, from 0 to 100")]
        time_scale: Option<f64>,
        #[arg(allow_negative_numbers = true, long, help = "From 0 to 16")]
        brightness: Option<f64>,
        #[arg(long, help = "Color that tints the particles, such as #ffcc88")]
        color: Option<String>,
        #[arg(long = "point", value_name = "X,Y", value_parser = pair, allow_hyphen_values = true, help = "A control point of this layer in pixels, p0 first; repeat it for the next ones")]
        points: Vec<[f64; 2]>,
        #[arg(long = "move-point", value_name = "K=X,Y", value_parser = indexed, allow_hyphen_values = true, conflicts_with = "points", help = "Move control point K of this layer only in pixels, the others staying as they are; repeat it for more")]
        moved: Vec<(usize, [f64; 2])>,
        #[arg(
            long = "unset-point",
            value_name = "K",
            conflicts_with = "points",
            help = "Draw control point K as the system does again"
        )]
        unset_points: Vec<usize>,
        #[arg(long, value_name = "VALUE", help = "Draw this value as the system does again")]
        unset: Vec<String>,
        #[arg(long, help = "Draw the layer as the system does in every value")]
        clear: bool,
    },
}

pub(crate) fn edit(layer: String, what: Stage) -> L {
    match what {
        Stage::Transform { position, rotation, scale, skew, origin, tilt, z, size, at, replace } => {
            L::Transform { layer, position, rotation, scale, skew, origin, tilt, z, size, at, replace }
        }
        Stage::Blend { blend } => L::Blend { layer, blend },
        Stage::Grip { grip, to, corner, even, snap, at, replace } => {
            let grip = match grip.as_str() {
                "scale" => json!({ "scale": { "corner": corner, "even": even } }),
                "skew" => json!({ "skew": { "edge": corner } }),
                other => json!(other),
            };
            let grip = serde_json::from_value(grip).unwrap_or(wprs_editor::actions::Grip::Move);
            L::Grip { layer, grip, to, snap, at, replace }
        }
        Stage::Fit { fit, at, replace } => {
            let fit = serde_json::from_value(json!(fit)).unwrap_or(wprs_editor::actions::Fit::Contain);
            L::Fit { layer, fit, at, replace }
        }
        Stage::Parent { parent, keep, at, replace } => L::Parent { layer, parent, keep, at, replace },
        Stage::Touch { takes, alpha } => L::Touch { layer, takes, alpha },
        Stage::Instance {
            alpha,
            size,
            count,
            speed,
            life,
            time_scale,
            brightness,
            color,
            points,
            moved,
            unset_points,
            unset,
            clear,
        } => {
            let numbers =
                [("alpha", alpha), ("size", size), ("count", count), ("speed", speed), ("life", life)];
            let more = [("time_scale", time_scale), ("brightness", brightness)];
            let mut values: Map<String, Value> =
                numbers.into_iter().chain(more).filter_map(|(k, v)| Some((k.into(), json!(v?)))).collect();
            color.iter().for_each(|c| drop(values.insert("color".into(), value(c))));
            if !points.is_empty() {
                values.insert("points".into(), json!(points));
            }
            let each = moved.into_iter().map(|(k, at)| (k, json!(at)));
            let each: Map<String, Value> = each
                .chain(unset_points.into_iter().map(|k| (k, Value::Null)))
                .map(|(k, v)| (k.to_string(), v))
                .collect();
            if !each.is_empty() {
                values.insert("points".into(), Value::Object(each));
            }
            unset.into_iter().for_each(|k| drop(values.insert(k, Value::Null)));
            L::Instance { layer, values, clear }
        }
    }
}
