#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - POEG Group %s Setting Register"]
    pub poegga: POEGG,
    _reserved1: [u8; 0xfc],
    #[doc = "0x100 - POEG Group %s Setting Register"]
    pub poeggb: POEGG,
    _reserved2: [u8; 0xfc],
    #[doc = "0x200 - POEG Group %s Setting Register"]
    pub poeggc: POEGG,
    _reserved3: [u8; 0xfc],
    #[doc = "0x300 - POEG Group %s Setting Register"]
    pub poeggd: POEGG,
}
#[doc = "POEGG (rw) register accessor: an alias for `Reg<POEGG_SPEC>`"]
pub type POEGG = crate::Reg<poegg::POEGG_SPEC>;
#[doc = "POEG Group %s Setting Register"]
pub mod poegg;
