use super::super::output::Output;
use super::super::state::State;
use super::slot::PageSlot;

const KEPT_LINES: usize = 50;

pub(super) fn keep(slot: &mut PageSlot, id: u32, line: String) {
    let Some((name, view)) = slot.views.iter_mut().find(|(_, v)| v.id == id) else { return };
    tracing::debug!("web page: {line}");
    if name.starts_with("preview:") || view.spec.as_ref().is_some_and(|s| s.drafted) {
        view.console.push_back(line);
        while view.console.len() > KEPT_LINES {
            view.console.pop_front();
        }
    }
}

impl State {
    pub(in crate::compositor) fn console<'a>(
        &'a self,
        name: &str,
        o: &'a Output,
    ) -> impl Iterator<Item = &'a String> {
        let virt = name.starts_with("preview:");
        let scene = o.shown.scene.as_ref().filter(|s| virt || s.spec.drafted);
        let scene = scene.into_iter().flat_map(|s| &s.console);
        let page = o.shown.content.page();
        let page = page.and_then(|p| self.pages.get(&p.key(self.power.choice)));
        let page = page
            .and_then(|p| p.views.get(name))
            .filter(|v| virt || v.spec.as_ref().is_some_and(|s| s.drafted));
        let page = page.into_iter().flat_map(|v| &v.console);
        scene.chain(page)
    }
}
