//! Undo/Redo transaction stack with coalescing and actor tracking.

use crate::actor::Actor;
use crate::command::Command;

/// Maximum allowable coalescing threshold interval in milliseconds.
pub const COALESCE_WINDOW_MS: i64 = 400;

/// Default history stack maximum depth.
pub const DEFAULT_MAX_DEPTH: usize = 500;

/// Single undoable transaction entry.
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryEntry {
    /// Forward command that was applied.
    pub command: Command,
    /// Inverse command that undoes the change.
    pub inverse: Command,
    /// Originating actor (Human, Agent, Hotkey, System).
    pub actor: Actor,
    /// Execution timestamp in epoch milliseconds.
    pub timestamp_ms: i64,
    /// Key used to detect repeated same-target actions for coalescing.
    pub coalesce_key: Option<String>,
}

/// History stack managing undo and redo buffers.
#[derive(Debug, Clone)]
pub struct HistoryStack {
    /// Stack of applied undoable operations.
    pub undo_stack: Vec<HistoryEntry>,
    /// Stack of undone operations available for redo.
    pub redo_stack: Vec<HistoryEntry>,
    /// Maximum allowed undo steps before discarding oldest.
    pub max_depth: usize,
}

impl Default for HistoryStack {
    fn default() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_depth: DEFAULT_MAX_DEPTH,
        }
    }
}

impl HistoryStack {
    /// Creates a new history stack with specified depth limit.
    pub fn new(max_depth: usize) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_depth,
        }
    }

    /// Records an executed command and its inverse onto the undo stack.
    ///
    /// If an identical target action occurs within 400 ms (e.g. continuous slider dragging
    /// or nudge key repeats), the entries coalesce into a single undo step.
    pub fn push(
        &mut self,
        command: Command,
        inverse: Command,
        actor: Actor,
        timestamp_ms: i64,
        coalesce_key: Option<String>,
    ) {
        // Clear redo stack on any new mutation
        self.redo_stack.clear();

        if let Some(ref key) = coalesce_key {
            if let Some(top) = self.undo_stack.last_mut() {
                if top.coalesce_key.as_ref() == Some(key)
                    && top.actor == actor
                    && (timestamp_ms - top.timestamp_ms).abs() <= COALESCE_WINDOW_MS
                {
                    // For relative Nudge operations, accumulate the inverse delta
                    if let (
                        Command::Nudge {
                            dx: inv_dx,
                            dy: inv_dy,
                            ..
                        },
                        Command::Nudge {
                            dx: new_dx,
                            dy: new_dy,
                            ..
                        },
                    ) = (&mut top.inverse, &inverse)
                    {
                        *inv_dx += *new_dx;
                        *inv_dy += *new_dy;
                    }

                    // Retain original inverse for absolute state updates, update forward command & timestamp
                    top.command = command;
                    top.timestamp_ms = timestamp_ms;
                    return;
                }
            }
        }

        self.undo_stack.push(HistoryEntry {
            command,
            inverse,
            actor,
            timestamp_ms,
            coalesce_key,
        });

        // Enforce maximum stack depth
        if self.undo_stack.len() > self.max_depth {
            self.undo_stack.remove(0);
        }
    }

    /// Pops the top undo operation, pushing it onto redo.
    pub fn undo(&mut self) -> Option<HistoryEntry> {
        if let Some(entry) = self.undo_stack.pop() {
            self.redo_stack.push(entry.clone());
            Some(entry)
        } else {
            None
        }
    }

    /// Pops the top redo operation, pushing it back onto undo.
    pub fn redo(&mut self) -> Option<HistoryEntry> {
        if let Some(entry) = self.redo_stack.pop() {
            self.undo_stack.push(entry.clone());
            Some(entry)
        } else {
            None
        }
    }

    /// Returns true if undo is currently available.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Returns true if redo is currently available.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Clears both undo and redo stacks.
    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}
