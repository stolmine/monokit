use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Live,
    Script1,
    Script2,
    Script3,
    Script4,
    Script5,
    Script6,
    Script7,
    Script8,
    Metro,
    Init,
    Pattern,
    Variables,
    Notes,
    Scope,
    Help,
}

pub const NAVIGABLE_PAGES: [Page; 15] = [
    Page::Live,
    Page::Script1,
    Page::Script2,
    Page::Script3,
    Page::Script4,
    Page::Script5,
    Page::Script6,
    Page::Script7,
    Page::Script8,
    Page::Metro,
    Page::Init,
    Page::Pattern,
    Page::Variables,
    Page::Notes,
    Page::Scope,
];

impl Page {
    pub fn name(&self) -> &str {
        match self {
            Page::Live => "LIVE",
            Page::Script1 => "1",
            Page::Script2 => "2",
            Page::Script3 => "3",
            Page::Script4 => "4",
            Page::Script5 => "5",
            Page::Script6 => "6",
            Page::Script7 => "7",
            Page::Script8 => "8",
            Page::Metro => "M",
            Page::Init => "I",
            Page::Pattern => "P",
            Page::Variables => "V",
            Page::Notes => "N",
            Page::Scope => "S",
            Page::Help => "HELP",
        }
    }

    pub fn next(&self) -> Self {
        if *self == Page::Help {
            return Page::Help;
        }
        let idx = NAVIGABLE_PAGES.iter().position(|p| p == self).unwrap_or(0);
        NAVIGABLE_PAGES[(idx + 1) % NAVIGABLE_PAGES.len()]
    }

    pub fn prev(&self) -> Self {
        if *self == Page::Help {
            return Page::Help;
        }
        let idx = NAVIGABLE_PAGES.iter().position(|p| p == self).unwrap_or(0);
        NAVIGABLE_PAGES[(idx + NAVIGABLE_PAGES.len() - 1) % NAVIGABLE_PAGES.len()]
    }
}

#[derive(Debug, Clone)]
pub struct ParamActivity {
    pub timestamps: [Option<Instant>; 48],
}

impl Default for ParamActivity {
    fn default() -> Self {
        Self {
            timestamps: [None; 48],
        }
    }
}

impl ParamActivity {
    pub fn mark(&mut self, param_name: &str) {
        if let Some(idx) = Self::param_to_index(param_name) {
            self.timestamps[idx] = Some(Instant::now());
        }
    }

    pub fn param_to_index(param: &str) -> Option<usize> {
        match param.to_lowercase().as_str() {
            "pf" => Some(0), "pw" => Some(1), "mf" => Some(2), "mw" => Some(3),
            "fm" => Some(4), "fb" => Some(5), "dc" => Some(6), "fc" => Some(7),
            "fq" => Some(8), "ft" => Some(9), "fe" => Some(10), "rf" => Some(11),
            "rd" => Some(12), "rm" => Some(13), "ad" => Some(14), "pd" => Some(15),
            "pa" => Some(16), "fd" => Some(17), "fa" => Some(18), "dd" => Some(19),
            "da" => Some(20), "dt" => Some(21), "df" => Some(22), "dw" => Some(23),
            "rv" => Some(24), "rh" => Some(25), "rw" => Some(26), "lm" => Some(27),
            "ct" => Some(28), "br_mix" | "br.mix" => Some(29),
            "ps_semi" | "ps.semi" => Some(30), "rgf" => Some(31),
            "lb" => Some(32), "ls" => Some(33), "eq" => Some(34), "tk" => Some(35),
            "mb" => Some(36), "mp" => Some(37), "md" => Some(38), "mt" => Some(39),
            "ma" => Some(40), "mx" => Some(41), "vol" | "volume" => Some(42),
            "pan" | "pn" => Some(43), "ds" => Some(44), "mm" => Some(45),
            "me" => Some(46), "fk" => Some(47),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchScope {
    Help,
    Script,
}

#[derive(Debug, Clone)]
pub struct SearchMatch {
    pub scope: SearchScope,
    pub page: Page,
    pub page_index: usize,
    pub line_index: usize,
    pub column_start: usize,
    pub column_end: usize,
    pub matched_text: String,
    pub context: String,
}

#[derive(Debug, Clone)]
pub struct ConditionalSegment {
    pub start: usize,
    pub end: usize,
    pub timestamp: Instant,
}

#[derive(Debug, Clone, Default)]
pub struct LineSegmentActivity {
    pub segments: Vec<ConditionalSegment>,
}

#[derive(Debug, Clone, Copy)]
pub struct OutputSettings {
    pub out_err: bool,
    pub out_ess: bool,
    pub out_qry: bool,
    pub out_cfm: bool,
}

impl Default for OutputSettings {
    fn default() -> Self {
        Self {
            out_err: false,
            out_ess: true,
            out_qry: true,
            out_cfm: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ScrambleSettings {
    pub scramble_enabled: bool,
    pub scramble_mode: u8,
    pub scramble_speed: u8,
    pub scramble_curve: u8,
}

#[derive(Debug, Clone)]
pub enum ConfirmAction {
    Quit,
    SaveOverwrite(String),
}

impl Default for ScrambleSettings {
    fn default() -> Self {
        Self {
            scramble_enabled: false,
            scramble_mode: 0,
            scramble_speed: 5,
            scramble_curve: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UIToggles {
    pub show_meters_header: bool,
    pub show_meters_grid: bool,
    pub show_spectrum: bool,
    pub show_activity: bool,
    pub show_grid: bool,
    pub show_grid_view: bool,
    pub show_seq_highlight: bool,
    pub show_conditional_highlight: bool,
}

impl Default for UIToggles {
    fn default() -> Self {
        Self {
            show_meters_header: true,
            show_meters_grid: true,
            show_spectrum: true,
            show_activity: true,
            show_grid: true,
            show_grid_view: false,
            show_seq_highlight: true,
            show_conditional_highlight: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorMode {
    TrueColor,
    Color256,
}
