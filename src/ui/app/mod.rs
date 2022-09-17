use crate::core::process::ProcData;

use self::state::AppState;

pub mod state;
pub mod widgets;

/// The main application, containing the state
pub struct App {
    /// State
    state: AppState,
    data: Vec<ProcData>
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new(data: Vec<ProcData>) -> Self {
        let state = AppState::default();
        let data = data;
        Self { state, data: data }
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn data(&self) -> &Vec<ProcData>{
        &self.data
    }
}