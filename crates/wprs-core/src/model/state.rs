use super::{Credit, GpuAdapter, HdrState, LayerShellState, OutputPlaylist, PreviewState, Uid};
use crate::{Filter, Fit, Screen};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::PathBuf;

model! {
    pub struct State {
        pub model: u32,
        pub version: String,
        pub adapter: String,
        #[serde(default)]
        pub tier: String,
        #[serde(default)]
        pub tier_measured: bool,
        pub config: PathBuf,
        pub config_error: Option<String>,
        pub library_error: Option<String>,
        pub serial: u64,
        pub paused: bool,
        pub on_battery: bool,
        #[serde(default)]
        pub battery_percent: Option<u8>,
        pub locked: bool,
        pub volume: f64,
        pub muted: bool,
        pub profile: Option<Uid>,
        pub jobs: u32,
        pub options: BTreeMap<String, Value>,
        pub outputs: BTreeMap<String, OutputState>,
        #[serde(default)]
        pub previews: BTreeMap<String, PreviewState>,
        #[serde(default)]
        pub calibrating: bool,
        #[serde(default)]
        pub compute: bool,
        #[serde(default)]
        pub adapters: Vec<GpuAdapter>,
        #[serde(default)]
        pub gpu_warning: Option<String>,
        #[serde(default)]
        pub gpu_note: Option<String>,
        #[serde(default)]
        pub gpu_restart: bool,
        #[serde(default)]
        pub qualities: BTreeMap<String, String>,
        #[serde(default)]
        pub hdrs: BTreeMap<String, String>,
        #[serde(default)]
        pub layer_shell: LayerShellState,
        #[serde(default)]
        pub plasma: bool,
    }

    pub struct OutputState {
        pub width: u32,
        pub height: u32,
        pub image: Option<PathBuf>,
        pub fit: Fit,
        #[serde(default)]
        pub background: String,
        pub kind: Shown,
        pub playing: bool,
        pub active: bool,
        pub covered: bool,
        #[serde(default)]
        pub still_reason: Option<StillReason>,
        pub clients: u32,
        pub error: Option<String>,
        pub item: Option<Uid>,
        pub preset: Option<Uid>,
        pub playlist: Option<OutputPlaylist>,
        pub credit: Option<Credit>,
        #[serde(default)]
        pub edited: bool,
        #[serde(default)]
        pub screen: Option<Screen>,
        #[serde(default)]
        pub mm: Option<[f64; 4]>,
        #[serde(default)]
        pub follows: Option<String>,
        #[serde(default)]
        pub parallax: bool,
        #[serde(default)]
        pub still: Option<crate::travel::Still>,
        #[serde(default)]
        pub frames: u64,
        #[serde(default)]
        pub page_frames: u64,
        #[serde(default)]
        pub refresh: u32,
        #[serde(default)]
        pub fps: u32,
        #[serde(default)]
        pub buffers: String,
        #[serde(default)]
        pub sound: String,
        #[serde(default)]
        pub shown_fit: Fit,
        #[serde(default)]
        pub flashes: String,
        #[serde(default)]
        pub left_out: Vec<String>,
        #[serde(default)]
        pub page_errors: Vec<String>,
        #[serde(default)]
        pub blank: bool,
        #[serde(default)]
        pub previous: bool,
        #[serde(default)]
        pub released: bool,
        #[serde(default)]
        pub particles: String,
        #[serde(default)]
        pub quality: String,
        #[serde(default)]
        pub quality_reason: String,
        #[serde(default)]
        pub quality_choice: Option<String>,
        #[serde(default)]
        pub quality_cost: f64,
        #[serde(default)]
        pub quality_budget: f64,
        #[serde(default)]
        pub quality_auto: String,
        #[serde(flatten)]
        pub quality_timing: QualityTiming,
        #[serde(default)]
        pub decode: String,
        #[serde(default)]
        pub drafted: bool,
        #[serde(default)]
        pub sandbox: String,
        #[serde(default)]
        pub helper: Option<u32>,
        #[serde(default)]
        pub accent: Option<String>,
        #[serde(default)]
        pub filter: Option<Filter>,
        #[serde(default)]
        pub hdr: HdrState,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub away: Option<String>,
    }

    #[derive(Copy, Eq, Hash)]
    pub enum StillReason { Covered, Battery, Locked, Paused }

    #[derive(Copy, Eq, Hash)]
    pub enum Shown { Color, Image, Video, Scene, Web, #[serde(other)] Unknown }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QualityTiming {
    #[serde(default)]
    pub quality_frame_ms: Option<f64>,
    #[serde(default)]
    pub quality_period_ms: Option<f64>,
    #[serde(default)]
    pub quality_step: String,
    #[serde(default)]
    pub quality_pace: u32,
}
