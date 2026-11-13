use crate::{init, make};
use gstreamer as gst;
use gstreamer::prelude::*;

/// Opens the plug-ins of hardware codecs now, under the loader lock held by `init`, instead of
/// lazily on whichever thread first needs one: they ask the graphics driver for its lock when
/// opened, which must never happen while another thread holds that lock and wants the loader's.
pub(crate) fn preload_hardware() {
    let registry = gst::Registry::get();
    let hardware = |feature: &gst::PluginFeature| {
        feature.downcast_ref::<gst::ElementFactory>().is_some_and(|f| f.klass().contains("Hardware"))
    };
    for feature in registry.features_filtered(hardware, false) {
        if let Err(e) = feature.plugin().map_or(Ok(()), |plugin| plugin.load().map(drop)) {
            tracing::debug!("hardware codec plug-in did not load: {e}");
        }
    }
}

/// Creates one element of each kind a first video needs, as opening a file does (tests of the
/// start-up order use it; it returns how many the machine has).
#[doc(hidden)]
pub fn create_first_elements() -> usize {
    let made = ["playbin", "decodebin", "matroskademux", "qtdemux", "videoconvert"]
        .map(|f| init().ok().and_then(|()| make(f).ok()));
    made.iter().flatten().count()
}

pub(crate) fn local_sources_only() {
    let registry = gst::Registry::get();
    let remote = |feature: &gst::PluginFeature| {
        feature.downcast_ref::<gst::ElementFactory>().is_some_and(|f| {
            let fetches = f.uri_type() == gst::URIType::Src
                && f.uri_protocols().iter().any(|p| !["file", "appsrc"].contains(&p.as_str()));
            fetches || ["Network", "Adaptive"].iter().any(|k| f.klass().contains(k))
        })
    };
    registry.features_filtered(remote, false).iter().for_each(|f| registry.remove_feature(f));
}
