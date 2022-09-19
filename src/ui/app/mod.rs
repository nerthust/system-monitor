use crate::core::process::ProcData;

use self::state::AppState;

pub mod state;
pub mod widgets;

/// The main application, containing the state
pub struct App {
    /// State
    state: AppState,
    data: Vec<ProcData>,
    tx_bits_n: u64,
    rx_bits_n: u64,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new(data: Vec<ProcData>, tx_n: u64, rx_n: u64) -> Self {
        let state = AppState::default();
        let data = data;
        let tx_bits_n = tx_n;
        let rx_bits_n = rx_n;
        Self { state, data, tx_bits_n, rx_bits_n }
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn data(&self) -> &Vec<ProcData>{
        &self.data
    }

    pub fn rx_bits(&self) -> &u64{
        &self.rx_bits_n
    }

    pub fn tx_bits(&self) -> &u64{
        &self.tx_bits_n
    }
}