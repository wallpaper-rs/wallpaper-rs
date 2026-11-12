use super::{State, turned};
use crate::scale::UNIT;
use crate::{Event, Placed};
use smithay_client_toolkit::output::OutputInfo;
use smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput;

pub(crate) fn name_of(info: &OutputInfo) -> String {
    info.name.clone().unwrap_or_else(|| format!("wayland-{}", info.id))
}

fn placed(info: &OutputInfo) -> Placed {
    let mode = info.modes.iter().find(|m| m.current);
    let logical = turned(info, info.logical_size.or(mode.map(|m| m.dimensions)).unwrap_or((0, 0)));
    let whole = info.scale_factor.max(1) as u32;
    let pixels = match mode {
        Some(m) => turned(info, m.dimensions),
        None => (logical.0 * whole, logical.1 * whole),
    };
    let scale = match logical.0 {
        0 => whole * UNIT,
        width => (u64::from(pixels.0) * u64::from(UNIT) + u64::from(width / 2)) as u32 / width,
    };
    Placed {
        logical,
        pixels,
        scale,
        refresh: mode.map_or(0, |m| m.refresh_rate.max(0) as u32),
        position: info.logical_position.unwrap_or(info.location),
        mm: turned(info, info.physical_size),
        identity: format!("{} {}", info.make, info.model).trim().to_owned(),
    }
}

impl State {
    pub(crate) fn rest(&mut self, output: WlOutput) {
        let Some(info) = self.outputs.info(&output) else { return };
        let name = name_of(&info);
        if !self.idle.iter().any(|(o, _)| *o == output) {
            self.idle.push((output, name.clone()));
        }
        (self.sink)(Event::Idle(name, placed(&info)));
    }

    pub(crate) fn rest_again(&mut self, output: &WlOutput) {
        if self.idle.iter().any(|(o, _)| o == output) {
            self.rest(output.clone());
        }
    }

    pub(crate) fn forget_idle(&mut self, output: &WlOutput) {
        if let Some(at) = self.idle.iter().position(|(o, _)| o == output) {
            let (_, name) = self.idle.remove(at);
            (self.sink)(Event::Gone(name));
        }
    }

    pub(crate) fn reconcile(&mut self) {
        let outputs: Vec<WlOutput> = self.outputs.outputs().collect();
        for output in outputs {
            let Some(info) = self.outputs.info(&output) else { continue };
            let want = (self.wanted)(&name_of(&info));
            let has = self.screens.iter().any(|s| s.output == output);
            tracing::debug!(output = name_of(&info), want, has, "checking the background surface");
            match (want, has) {
                (true, false) => self.add(output),
                (false, true) => {
                    self.screens.retain(|s| s.output != output);
                    self.rest(output);
                }
                (true, true) => self.changed(|s| s.output == output, |_| {}),
                (false, false) => {}
            }
        }
    }
}
