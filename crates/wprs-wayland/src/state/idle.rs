use super::{State, turned};
use crate::scale::UNIT;
use std::time::{Duration, Instant};

const REMAP_WAIT: Duration = Duration::from_millis(300);
use crate::{Event, Placed};
use smithay_client_toolkit::output::OutputInfo;
use smithay_client_toolkit::reexports::client::protocol::wl_output::WlOutput;
use smithay_client_toolkit::shell::WaylandSurface;

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
    pub(crate) fn rest(&mut self, output: &WlOutput) {
        let Some(info) = self.outputs.info(output) else { return };
        (self.sink)(Event::Idle(name_of(&info), placed(&info)));
    }

    pub(crate) fn reconcile(&mut self) {
        let outputs: Vec<WlOutput> = self.outputs.outputs().collect();
        for output in outputs {
            let Some(info) = self.outputs.info(&output) else { continue };
            let want = (self.wanted)(&name_of(&info));
            let Some(at) = self.screens.iter().position(|s| s.output == output) else {
                self.add(output);
                continue;
            };
            let screen = &mut self.screens[at];
            match (want, screen.mapped) {
                (true, true) => self.changed(|s| s.output == output, |_| {}),
                (true, false) => {
                    screen.mapped = true;
                    if std::mem::take(&mut screen.cold) {
                        screen.awaiting = Some(Instant::now() + REMAP_WAIT);
                        screen.parts.layer.commit();
                    } else {
                        self.changed(|s| s.output == output, |_| {});
                    }
                }
                (false, true) => {
                    screen.mapped = false;
                    screen.cold = true;
                    let surface = screen.parts.layer.wl_surface();
                    surface.attach(None, 0, 0);
                    surface.commit();
                    self.rest(&output);
                }
                (false, false) => {}
            }
        }
    }

    pub(crate) fn remap_at(&self) -> Option<Instant> {
        self.screens.iter().filter_map(|s| s.awaiting).min()
    }

    pub(crate) fn remap_due(&mut self) {
        let now = Instant::now();
        let due: Vec<WlOutput> = self
            .screens
            .iter()
            .filter(|s| s.awaiting.is_some_and(|at| at <= now))
            .map(|s| s.output.clone())
            .collect();
        for output in due {
            self.changed(|s| s.output == output, |s| s.awaiting = None);
        }
    }
}
