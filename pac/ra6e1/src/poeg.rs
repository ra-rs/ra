#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - POEG Group A Setting Register"]
    pub poegga: POEGGA,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100 - POEG Group B Setting Register"]
    pub poeggb: POEGGB,
    _reserved2: [u8; 0xfc],
    #[doc = "0x200 - POEG Group C Setting Register"]
    pub poeggc: POEGGC,
    _reserved3: [u8; 0xfc],
    #[doc = "0x300 - POEG Group D Setting Register"]
    pub poeggd: POEGGD,
}
#[doc = "POEGGA (rw) register accessor: an alias for `Reg<POEGGA_SPEC>`"]
pub type POEGGA = crate::Reg<poegga::POEGGA_SPEC>;
#[doc = "POEG Group A Setting Register"]
pub mod poegga;
#[doc = "POEGGB (rw) register accessor: an alias for `Reg<POEGGB_SPEC>`"]
pub type POEGGB = crate::Reg<poeggb::POEGGB_SPEC>;
#[doc = "POEG Group B Setting Register"]
pub mod poeggb;
#[doc = "POEGGC (rw) register accessor: an alias for `Reg<POEGGC_SPEC>`"]
pub type POEGGC = crate::Reg<poeggc::POEGGC_SPEC>;
#[doc = "POEG Group C Setting Register"]
pub mod poeggc;
#[doc = "POEGGD (rw) register accessor: an alias for `Reg<POEGGD_SPEC>`"]
pub type POEGGD = crate::Reg<poeggd::POEGGD_SPEC>;
#[doc = "POEG Group D Setting Register"]
pub mod poeggd;
