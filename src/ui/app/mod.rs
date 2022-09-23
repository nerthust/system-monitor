use crate::core::process::ProcData;

pub mod widgets;

// The main application, containing the state
pub struct App {
    // All data per process
    data: Vec<ProcData>,
    // Network bits transmited
    tx_bits_n: u64,
    // Network bits received
    rx_bits_n: u64,
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new(data: Vec<ProcData>, tx_n: u64, rx_n: u64) -> Self {
        let data = data;
        let tx_bits_n = tx_n;
        let rx_bits_n = rx_n;
        Self {
            data,
            tx_bits_n,
            rx_bits_n,
        }
    }

    pub fn data(&mut self) -> &Vec<ProcData> {
        &self.data
    }

    pub fn update_data(&mut self, data: &Vec<ProcData>) -> () {
        self.data = data.to_vec();
    }

    pub fn rx_bits(&self) -> &u64 {
        &self.rx_bits_n
    }

    pub fn update_rx_bits(&mut self, rx_bits: u64) -> () {
        self.rx_bits_n = rx_bits;
    }

    pub fn tx_bits(&self) -> &u64 {
        &self.tx_bits_n
    }

    pub fn update_tx_bits(&mut self, tx_bits: u64) -> () {
        self.tx_bits_n = tx_bits;
    }
}
