use super::super::output::Output;
use super::super::state::State;
use super::slot::PageSlot;

const KEPT_LINES: usize = 50;

pub(super) fn keep(slot: &mut PageSlot, id: u32, line: String) {
    let Some((_, view)) = slot.views.iter_mut().find(|(_, v)| v.id == id) else { return };
    tracing::debug!("web page: {line}");
    view.console.push_back(line);
    while view.console.len() > KEPT_LINES {
        view.console.pop_front();
    }
}

const LOGGED: [&str; 4] = ["log: ", "info: ", "warn: ", "debug: "];

pub(crate) fn is_problem(line: &str) -> bool {
    !LOGGED.iter().any(|level| line.starts_with(level))
}

pub(crate) fn scene_problem(line: &str) -> bool {
    line.starts_with("error ")
}

impl State {
    pub(in crate::compositor) fn page_problems(&self, name: &str, o: &Output) -> Vec<String> {
        let page = o.shown.content.page();
        let page = page.and_then(|p| self.pages.get(&p.key(self.power.choice)));
        let view = page.and_then(|p| p.views.get(name));
        let scene = o.shown.scene.iter().flat_map(|s| &s.console).filter(|l| scene_problem(l));
        let page = view.into_iter().flat_map(|v| &v.console).filter(|l| is_problem(l));
        page.chain(scene).cloned().collect()
    }

    pub(in crate::compositor) fn page_package(&self, o: &Output) -> Option<std::path::PathBuf> {
        let page = o.shown.content.page();
        let page = page.and_then(|p| self.pages.get(&p.key(self.power.choice)));
        page.map(|p| p.spec.pack.path.clone())
    }

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

#[cfg(test)]
mod tests {
    use super::{is_problem, scene_problem};

    #[test]
    fn only_error_lines_of_a_scene_script_are_problems() {
        assert!(scene_problem("error scripts/a.js: TypeError: x is not a function"));
        for line in ["log scripts/a.js: hi", "warn scripts/a.js: careful", "errors are fine: no"] {
            assert!(!scene_problem(line), "{line}");
        }
    }

    #[test]
    fn only_errors_of_a_page_are_problems_and_its_plain_logging_is_not() {
        for line in ["RangeError: Incorrect locale information provided", "error: failed", "TypeError: x"] {
            assert!(is_problem(line), "{line}");
        }
        for line in ["log: hello", "info: a", "warn: b", "debug: c"] {
            assert!(!is_problem(line), "{line}");
        }
    }
}
