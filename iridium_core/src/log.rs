use std::collections::VecDeque;

use iridium_ecs::{
    storage::{ComponentStorage, StoredComponentField},
    ui::InspectorUi,
};
use iridium_ecs_macros::{Component, HasStableTypeId};
use iridium_map_utils::fast_map;

/// The type of log message.
pub enum LogType {
    /// Just debug info.
    Info,
    /// Something went wrong,
    /// but the program can continue.
    Warning,
    /// Something went wrong,
    /// and some code halted as a result.
    Error,
}

/// An entry in the log.
pub struct LogEntry {
    /// When the message was logged.
    pub timestamp: std::time::Instant,
    /// The message.
    pub message: String,
    /// The type of the message.
    pub log_type: LogType,
}

impl LogEntry {
    /// Create a new log entry.
    #[must_use]
    pub fn new(message: impl Into<String>, log_type: LogType) -> Self {
        Self {
            timestamp: std::time::Instant::now(),
            message: message.into(),
            log_type,
        }
    }
}

/// Stores the log.
#[derive(Component, HasStableTypeId)]
pub struct LogState {
    /// The log entries.
    entries: VecDeque<LogEntry>,
    /// The maximum number of entries.
    max_entries: usize,
}

impl InspectorUi for LogState {
    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label(format!("There are {} logs", self.entries().count()));
        ui.end_row();
        ui.label("See log panel for more");
    }
}

impl Default for LogState {
    fn default() -> Self {
        Self::new(100)
    }
}

impl ComponentStorage for LogState {
    fn from_stored(
        mut stored: iridium_ecs::storage::StoredComponent,
        _assets: &iridium_assets::Assets,
    ) -> Option<Self> {
        Some(Self::new(stored.get("max_entries")?.parse().ok()?))
    }

    fn to_stored(&self) -> iridium_ecs::storage::StoredComponent {
        iridium_ecs::storage::StoredComponent {
            type_name: "LogState".to_string(),
            fields: fast_map! {
                "max_entries" => StoredComponentField::new(self.max_entries.to_string(), false),
            },
        }
    }
}

impl LogState {
    /// Creates a new `LogState` with the given maximum number of entries.
    #[must_use]
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: VecDeque::with_capacity(max_entries),
            max_entries,
        }
    }

    /// Logs a message.
    pub fn log(&mut self, entry: LogEntry) {
        if self.entries.len() >= self.max_entries {
            self.entries.pop_front();
        }

        self.entries.push_back(entry);
    }

    /// Logs an info message.
    pub fn info(&mut self, message: impl Into<String>) {
        self.log(LogEntry::new(message, LogType::Info));
    }

    /// Logs a warning message.
    pub fn warning(&mut self, message: impl Into<String>) {
        self.log(LogEntry::new(message, LogType::Warning));
    }

    /// Logs an error message.
    pub fn error(&mut self, message: impl Into<String>) {
        self.log(LogEntry::new(message, LogType::Error));
    }

    /// Returns an iterator over the log entries.
    pub fn entries(&self) -> impl Iterator<Item = &LogEntry> {
        self.entries.iter()
    }
}
