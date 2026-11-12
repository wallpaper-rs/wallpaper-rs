use super::{Descriptor, ParamDef, ParamKind, Solve, Unit};
use crate::value::Param;

const POINTS: [&str; 3] = ["center", "source", "pivot"];
const ENDS: [(&str, &str); 4] = [("from", "to"), ("start", "end"), ("a", "b"), ("first", "second")];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grab {
    Point,
    Corner,
    Vector,
    Angle,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hold<'a> {
    pub param: &'a str,
    pub kind: Grab,
    pub default: [f32; 2],
    pub range: [f32; 2],
    pub partner: Option<&'a str>,
}

fn value(p: &ParamDef) -> [f32; 2] {
    match &p.default {
        Param::Vec2(v) => *v,
        Param::Number(n) => [*n, *n],
        _ => [0.5, 0.5],
    }
}

fn unit_vector(p: &ParamDef) -> bool {
    p.kind == ParamKind::Vec2 && p.min == Some(-1.0) && p.max == Some(1.0)
}

fn point(p: &ParamDef) -> bool {
    p.kind == ParamKind::Position || (p.kind == ParamKind::Vec2 && POINTS.contains(&p.id.as_str()))
}

fn ends<'a>(params: &'a [ParamDef], p: &ParamDef) -> Option<&'a str> {
    ENDS.iter().find_map(|(a, b)| {
        let (stem, other) = match (p.id.strip_suffix(a), p.id.strip_suffix(b)) {
            (Some(stem), _) => (stem, b),
            (_, Some(stem)) => (stem, a),
            _ => return None,
        };
        let wanted = format!("{stem}{other}");
        params.iter().find(|q| q.id == wanted && point(q)).map(|q| q.id.as_str())
    })
}

fn make<'a>(p: &'a ParamDef, kind: Grab, range: [f32; 2], partner: Option<&'a str>) -> Hold<'a> {
    Hold { param: &p.id, kind, default: value(p), range, partner }
}

fn one<'a>(params: &'a [ParamDef], p: &'a ParamDef) -> Option<Hold<'a>> {
    if point(p) {
        return Some(make(p, Grab::Point, [-2.0, 4.0], ends(params, p)));
    }
    if unit_vector(p) {
        return Some(make(p, Grab::Vector, [-1.0, 1.0], None));
    }
    let degrees = p.kind == ParamKind::Float && p.unit == Some(Unit::Degrees);
    degrees.then(|| make(p, Grab::Angle, [p.min.unwrap_or(-360.0), p.max.unwrap_or(360.0)], None))
}

impl Descriptor {
    pub fn handles(&self) -> Vec<Hold<'_>> {
        if self.solve == Some(Solve::Homography) {
            return self.params.iter().take(4).map(|p| make(p, Grab::Corner, [-2.0, 4.0], None)).collect();
        }
        self.params.iter().filter_map(|p| one(&self.params, p)).collect()
    }
}
