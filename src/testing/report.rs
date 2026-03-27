/// Summary produced by `PlayTester::run_agent_report`.
#[derive(Debug, Default)]
pub struct TestReport {
    /// Number of ticks executed before the run ended.
    pub turns_run: u32,
    /// Whether `game_over` was set during the run.
    pub game_over: bool,
    /// Whether a Rust panic occurred during the run (caught via `catch_unwind`).
    pub panicked: bool,
    /// Free-form notes accumulated during the run (e.g. messages from the game).
    pub notes: Vec<String>,
}

impl TestReport {
    pub fn new() -> Self {
        TestReport::default()
    }

    pub fn add_note(&mut self, note: impl Into<String>) {
        self.notes.push(note.into());
    }
}
