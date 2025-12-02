use crate::types::{Page, ScriptStorage, SearchMatch, SearchScope};
use crate::ui::pages::help::HelpCategory;

pub fn search_help(query: &str, help_categories: &[HelpCategory]) -> Vec<SearchMatch> {
    if query.is_empty() {
        return Vec::new();
    }

    let mut matches = Vec::new();
    let query_lower = query.to_lowercase();

    for (page_index, category) in help_categories.iter().enumerate() {
        for (line_index, line) in category.lines.iter().enumerate() {
            let line_lower = line.to_lowercase();
            let mut start = 0;

            while let Some(pos) = line_lower[start..].find(&query_lower) {
                let absolute_pos = start + pos;
                let column_start = absolute_pos;
                let column_end = absolute_pos + query.len();
                let matched_text = line[column_start..column_end].to_string();

                matches.push(SearchMatch {
                    scope: SearchScope::Help,
                    page: Page::Help,
                    page_index,
                    line_index,
                    column_start,
                    column_end,
                    matched_text,
                    context: line.to_string(),
                });

                start = absolute_pos + 1;
            }
        }
    }

    matches
}

pub fn search_scripts(query: &str, scripts: &ScriptStorage) -> Vec<SearchMatch> {
    if query.is_empty() {
        return Vec::new();
    }

    let mut matches = Vec::new();
    let query_lower = query.to_lowercase();

    for (script_index, script) in scripts.scripts.iter().enumerate() {
        let page = match script_index {
            0 => Page::Script1,
            1 => Page::Script2,
            2 => Page::Script3,
            3 => Page::Script4,
            4 => Page::Script5,
            5 => Page::Script6,
            6 => Page::Script7,
            7 => Page::Script8,
            8 => Page::Metro,
            9 => Page::Init,
            _ => continue,
        };

        for (line_index, line) in script.lines.iter().enumerate() {
            let line_lower = line.to_lowercase();
            let mut start = 0;

            while let Some(pos) = line_lower[start..].find(&query_lower) {
                let absolute_pos = start + pos;
                let column_start = absolute_pos;
                let column_end = absolute_pos + query.len();
                let matched_text = line[column_start..column_end].to_string();

                matches.push(SearchMatch {
                    scope: SearchScope::Script,
                    page,
                    page_index: script_index,
                    line_index,
                    column_start,
                    column_end,
                    matched_text,
                    context: line.to_string(),
                });

                start = absolute_pos + 1;
            }
        }
    }

    matches
}
