pub enum State {
    Unknown,
    Charging,
    Discharging = 42,
    Full,
    __Nonexhaustive = 255,
}

#[repr(transparent)]
pub struct CState(pub u8);

impl CState {
    pub const UNKNOWN: Self = Self(State::Unknown as u8);
    pub const CHARGING: Self = Self(State::Charging as u8);
    pub const DISCHARGING: Self = Self(State::Discharging as u8);
    pub const FULL: Self = Self(State::Full as u8);
}

#[no_mangle]
pub extern "C" fn battery_get_state() -> CState {
    CState::UNKNOWN
}

#[repr(u8)]
pub enum Technology {
    Unknown,
    LithiumIon,
    LeadAcid = 7,
}

#[repr(transparent)]
pub struct CTechnology(pub u8);

impl CTechnology {
    pub const UNKNOWN: Self = Self(Technology::Unknown as u8);
    pub const LITHIUM_ION: Self = Self(Technology::LithiumIon as u8);
    pub const LEAD_ACID: Self = Self(Technology::LeadAcid as u8);
}

#[no_mangle]
pub extern "C" fn battery_get_technology() -> CTechnology {
    CTechnology::UNKNOWN
}
