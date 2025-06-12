use super::*;

// Common trait for events
pub trait Event: Send + Sync {
    fn describe(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct CreateTokenEvent {
    address: String,
}

impl CreateTokenEvent {
    pub fn new() -> Self {
        Self {
            address: format!("0x{}", hex::encode(random::<u64>().to_string())),
        }
    }
}

impl Event for CreateTokenEvent {
    fn describe(&self) -> String {
        format!("CreateTokenEvent with address: {}", self.address)
    }
}

#[derive(Debug, Clone)]
pub struct MintEvent {
    address: String,
    amount: u64,
}

impl MintEvent {
    pub fn new() -> Self {
        Self {
            address: format!("0x{}", hex::encode(random::<u64>().to_string())),
            amount: random::<u64>(),
        }
    }
}

impl Event for MintEvent {
    fn describe(&self) -> String {
        format!(
            "MintEvent with address: {} and amount: {}",
            self.address, self.amount
        )
    }
}

#[derive(Debug, Clone)]
pub struct BurnEvent {
    address: String,
    amount: u64,
}

impl BurnEvent {
    pub fn new() -> Self {
        Self {
            address: format!("0x{}", hex::encode(random::<u64>().to_string())),
            amount: random::<u64>(),
        }
    }
}

impl Event for BurnEvent {
    fn describe(&self) -> String {
        format!(
            "BurnEvent with address: {} and amount: {}",
            self.address, self.amount
        )
    }
}

#[derive(Debug, Clone)]
pub struct TransferEvent {
    from: String,
    to: String,
    amount: u64,
}

impl TransferEvent {
    pub fn new() -> Self {
        Self {
            from: format!("0x{}", hex::encode(random::<u64>().to_string())),
            to: format!("0x{}", hex::encode(random::<u64>().to_string())),
            amount: random::<u64>(),
        }
    }
}

impl Event for TransferEvent {
    fn describe(&self) -> String {
        format!(
            "TransferEvent with from: {} to: {} with amount: {}",
            self.from, self.to, self.amount
        )
    }
}

#[derive(Debug, Clone)]
pub struct FreezeEvent {
    address: String,
}

impl FreezeEvent {
    pub fn new() -> Self {
        Self {
            address: format!("0x{}", hex::encode(random::<u64>().to_string())),
        }
    }
}

impl Event for FreezeEvent {
    fn describe(&self) -> String {
        format!("FreezeEvent with address: {}", self.address)
    }
}
