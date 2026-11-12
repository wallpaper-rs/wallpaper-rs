use super::search::Search;
use reqwest::Url;

fn pasted(text: &str) -> Option<Url> {
    let url = Url::parse(text.trim()).ok()?;
    let host = url.host_str()?.trim_start_matches("www.");
    (url.scheme() == "https" && host == "wallhaven.cc" && url.path().trim_end_matches('/') == "/search")
        .then_some(url)
}

fn split(list: &str) -> Vec<String> {
    list.split(',')
        .map(|item| item.trim().trim_start_matches('#').to_lowercase())
        .filter(|i| !i.is_empty())
        .collect()
}

impl Search {
    pub fn is_search_link(text: &str) -> bool {
        pasted(text).is_some()
    }

    pub fn apply_link(&mut self, text: &str) -> bool {
        let Some(url) = pasted(text) else { return false };
        self.query = None;
        for (key, value) in url.query_pairs() {
            let value = value.trim().to_owned();
            match key.as_ref() {
                "q" => self.take_terms(&value),
                "categories" => self.categories = value,
                "purity" => self.purity = value,
                "sorting" => self.sorting = value,
                "order" => self.order = value,
                "topRange" => self.top_range = value,
                "atleast" => self.atleast = Some(value).filter(|v| !v.is_empty()),
                "resolutions" => self.resolutions = split(&value),
                "ratios" => self.ratios = split(&value),
                "colors" => self.colors = split(&value),
                "page" => self.page = value.parse().unwrap_or(1).max(1),
                _ => {}
            }
        }
        true
    }

    fn take_terms(&mut self, terms: &str) {
        if let Some(id) = terms.strip_prefix("like:") {
            self.similar = Some(id.trim().to_owned());
        } else if let Some(id) = terms.strip_prefix("id:") {
            self.tag = Some(id.trim().to_owned());
        } else {
            self.query = Some(terms.to_owned()).filter(|terms| !terms.is_empty());
        }
    }
}
