#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0xc0],
    #[doc = "0xc0 - D/A A/D Synchronous Unit Select Register"]
    pub daadusr: DAADUSR,
}
#[doc = "DAADUSR (rw) register accessor: an alias for `Reg<DAADUSR_SPEC>`"]
pub type DAADUSR = crate::Reg<daadusr::DAADUSR_SPEC>;
#[doc = "D/A A/D Synchronous Unit Select Register"]
pub mod daadusr;
