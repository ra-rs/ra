#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator Control Register"]
    pub cmpctl: CMPCTL,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - Comparator Input Select Register"]
    pub cmpsel0: CMPSEL0,
    _reserved2: [u8; 0x03],
    #[doc = "0x08 - Comparator Reference Voltage Select Register"]
    pub cmpsel1: CMPSEL1,
    _reserved3: [u8; 0x03],
    #[doc = "0x0c - Comparator Output Monitor Register"]
    pub cmpmon: CMPMON,
    _reserved4: [u8; 0x03],
    #[doc = "0x10 - Comparator Output Control Register"]
    pub cpioc: CPIOC,
}
#[doc = "CMPCTL (rw) register accessor: an alias for `Reg<CMPCTL_SPEC>`"]
pub type CMPCTL = crate::Reg<cmpctl::CMPCTL_SPEC>;
#[doc = "Comparator Control Register"]
pub mod cmpctl;
#[doc = "CMPSEL0 (rw) register accessor: an alias for `Reg<CMPSEL0_SPEC>`"]
pub type CMPSEL0 = crate::Reg<cmpsel0::CMPSEL0_SPEC>;
#[doc = "Comparator Input Select Register"]
pub mod cmpsel0;
#[doc = "CMPSEL1 (rw) register accessor: an alias for `Reg<CMPSEL1_SPEC>`"]
pub type CMPSEL1 = crate::Reg<cmpsel1::CMPSEL1_SPEC>;
#[doc = "Comparator Reference Voltage Select Register"]
pub mod cmpsel1;
#[doc = "CMPMON (r) register accessor: an alias for `Reg<CMPMON_SPEC>`"]
pub type CMPMON = crate::Reg<cmpmon::CMPMON_SPEC>;
#[doc = "Comparator Output Monitor Register"]
pub mod cmpmon;
#[doc = "CPIOC (rw) register accessor: an alias for `Reg<CPIOC_SPEC>`"]
pub type CPIOC = crate::Reg<cpioc::CPIOC_SPEC>;
#[doc = "Comparator Output Control Register"]
pub mod cpioc;
