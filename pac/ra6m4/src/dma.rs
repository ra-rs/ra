#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Module Activation Register"]
    pub dmast: DMAST,
    _reserved1: [u8; 0x3f],
    #[doc = "0x40 - DMAC Error Channel Register"]
    pub dmechr: DMECHR,
}
#[doc = "DMAST (rw) register accessor: an alias for `Reg<DMAST_SPEC>`"]
pub type DMAST = crate::Reg<dmast::DMAST_SPEC>;
#[doc = "DMA Module Activation Register"]
pub mod dmast;
#[doc = "DMECHR (rw) register accessor: an alias for `Reg<DMECHR_SPEC>`"]
pub type DMECHR = crate::Reg<dmechr::DMECHR_SPEC>;
#[doc = "DMAC Error Channel Register"]
pub mod dmechr;
