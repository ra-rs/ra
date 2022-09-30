#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - POEG Group A Setting Register"]
    pub poegga: POEGGA,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100 - POEG Group B Setting Register"]
    pub poeggb: POEGGB,
}
#[doc = "POEGGA (rw) register accessor: an alias for `Reg<POEGGA_SPEC>`"]
pub type POEGGA = crate::Reg<poegga::POEGGA_SPEC>;
#[doc = "POEG Group A Setting Register"]
pub mod poegga;
#[doc = "POEGGB (rw) register accessor: an alias for `Reg<POEGGB_SPEC>`"]
pub type POEGGB = crate::Reg<poeggb::POEGGB_SPEC>;
#[doc = "POEG Group B Setting Register"]
pub mod poeggb;
