/// Edit actions for undo/redo functionality
/// Each script page and notes page has its own undo/redo stack

#[derive(Clone, Debug)]
pub enum EditAction {
    // Script operations
    SaveLine {
        line_idx: usize,
        old: String,
        new: String,
    },
    DeleteLine {
        line_idx: usize,
        content: String,
    },
    DuplicateLine {
        line_idx: usize,
        shifted_lines: Vec<String>,
    },
    CutLine {
        line_idx: usize,
        content: String,
    },
    PasteLine {
        line_idx: usize,
        old: String,
        new: String,
    },

    // Notes operations (same structure, different context)
    SaveNotesLine {
        line_idx: usize,
        old: String,
        new: String,
    },
    DeleteNotesLine {
        line_idx: usize,
        content: String,
    },
    DuplicateNotesLine {
        line_idx: usize,
        shifted_lines: Vec<String>,
    },
    CutNotesLine {
        line_idx: usize,
        content: String,
    },
    PasteNotesLine {
        line_idx: usize,
        old: String,
        new: String,
    },
}

const MAX_UNDO_STACK_SIZE: usize = 50;

/// Manages undo/redo stacks for a single page (script or notes)
#[derive(Clone, Debug, Default)]
pub struct UndoStack {
    pub undo: Vec<EditAction>,
    pub redo: Vec<EditAction>,
}

impl UndoStack {
    pub fn new() -> Self {
        Self {
            undo: Vec::new(),
            redo: Vec::new(),
        }
    }

    pub fn push(&mut self, action: EditAction) {
        self.undo.push(action);
        self.redo.clear();
        if self.undo.len() > MAX_UNDO_STACK_SIZE {
            self.undo.remove(0);
        }
    }

    pub fn pop_undo(&mut self) -> Option<EditAction> {
        self.undo.pop()
    }

    pub fn push_redo(&mut self, action: EditAction) {
        self.redo.push(action);
    }

    pub fn pop_redo(&mut self) -> Option<EditAction> {
        self.redo.pop()
    }

    pub fn push_undo_from_redo(&mut self, action: EditAction) {
        self.undo.push(action);
        if self.undo.len() > MAX_UNDO_STACK_SIZE {
            self.undo.remove(0);
        }
    }

    pub fn clear(&mut self) {
        self.undo.clear();
        self.redo.clear();
    }
}
