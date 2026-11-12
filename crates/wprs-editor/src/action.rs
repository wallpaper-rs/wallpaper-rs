use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

mod declared;
mod exports;
mod glows;
mod grips;
mod ids;
mod lamps;
mod limbs;
mod marks;
mod motion;
mod names;
mod pools;
mod rigging;
mod shaping;
mod solid;
mod sow;
mod stage2d;
mod swarm;
mod tend;
mod timing;
pub use exports::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    Set {
        pointer: String,
        value: Value,
    },
    Add {
        pointer: String,
        value: Value,
    },
    Remove {
        pointer: String,
    },
    Move {
        from: String,
        to: String,
    },
    AddLayer {
        kind: String,
        #[serde(default)]
        id: Option<String>,
        #[serde(default)]
        name: Option<String>,
        #[serde(default)]
        source: Option<String>,
        #[serde(default)]
        into: Option<String>,
        #[serde(default)]
        at: Option<usize>,
        #[serde(default)]
        file: Option<PathBuf>,
    },
    RemoveLayer {
        id: String,
    },
    DuplicateLayer {
        id: String,
        #[serde(default)]
        template: bool,
    },
    MoveLayer {
        id: String,
        to: usize,
        #[serde(default)]
        into: Option<String>,
    },
    Group {
        ids: Vec<String>,
        #[serde(default)]
        id: Option<String>,
    },
    Ungroup {
        id: String,
    },
    Hide {
        id: String,
        hidden: bool,
    },
    Lock {
        id: String,
        locked: bool,
    },
    Solo {
        #[serde(default)]
        id: Option<String>,
    },
    Keyframe(KeyframeEdit),
    Import {
        file: PathBuf,
        #[serde(default)]
        id: Option<String>,
        #[serde(default)]
        linear: bool,
        #[serde(default)]
        atlas: Option<[u32; 2]>,
    },
    ImportEffect {
        descriptor: PathBuf,
        #[serde(default)]
        wgsl: Option<PathBuf>,
    },
    RemoveEffect {
        effect: String,
    },
    ImportScript(ScriptFile),
    ImportAssets(ImportAssets),
    Asset(AssetEdit),
    Rgb(RgbEdit),
    Translate(TranslateEdit),
    Rename(RenameEdit),
    Paste(PasteEdit),
    Thumbnail {
        file: PathBuf,
        #[serde(default)]
        animated: bool,
    },
    Upgrade(UpgradeEdit),
    AddProperty {
        kind: String,
        #[serde(default)]
        id: Option<String>,
        #[serde(default)]
        group: Option<String>,
        #[serde(default)]
        label: Option<String>,
    },
    MoveProperty {
        id: String,
        to: usize,
        #[serde(default)]
        group: Option<String>,
    },
    RenameProperty {
        id: String,
        to: String,
    },
    RemoveProperty {
        id: String,
    },
    Condition(ConditionEdit),
    EffectRule(RuleEdit),
    AddPreset {
        #[serde(default)]
        id: Option<String>,
        #[serde(default)]
        label: Option<String>,
    },
    PaintMask(MaskStroke),
    MeshFromPicture(MeshCut),
    Bone(BoneEdit),
    MeshDepth(DepthStroke),
    Weights(WeightStroke),
    AutoWeights(AutoWeights),
    Clip(ClipEdit),
    Key(ClipKey),
    Expression(ExpressionEdit),
    Ik(IkEdit),
    Physics(BodyEdit),
    BoneRule(BoneRuleEdit),
    Wind(WindEdit),
    Attach(AttachEdit),
    Point(PointEdit),
    Parts(PartsEdit),
    Particles(ParticleEdit),
    Light(LightEdit),
    Lighting(LightingEdit),
    Normals(NormalsMake),
    PaintMap(MapStroke),
    Ambient(AmbientEdit),
    Scene3d(Edit3d),
    Animate(AnimEdit),
    Effects(EffectsEdit),
    Layer(LayerEdit),
    Manifest(ManifestEdit),
    Property(PropertyEdit),
    Canvas(CanvasEdit),
    Timeline(TimelineEdit),
    Script(ScriptEdit),
    Template(TemplateEdit),
    Pool(PoolEdit),
    Uses(UsesEdit),
    Uses(UsesEdit),
    Undo {},
    Redo {},
}
