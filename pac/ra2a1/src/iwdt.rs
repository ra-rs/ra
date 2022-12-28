#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IWDT Refresh Register"]
    pub iwdtrr: IWDTRR,
    _reserved1: [u8; 0x03],
    #[doc = "0x04 - IWDT Status Register"]
    pub iwdtsr: IWDTSR,
}
#[doc = "IWDTRR (rw) register accessor: an alias for `Reg<IWDTRR_SPEC>`"]
pub type IWDTRR = crate::Reg<iwdtrr::IWDTRR_SPEC>;
#[doc = "IWDT Refresh Register"]
pub mod iwdtrr;
#[doc = "IWDTSR (rw) register accessor: an alias for `Reg<IWDTSR_SPEC>`"]
pub type IWDTSR = crate::Reg<iwdtsr::IWDTSR_SPEC>;
#[doc = "IWDT Status Register"]
pub mod iwdtsr;
