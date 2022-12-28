#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMAC Module Activation Register"]
    pub dmast: DMAST,
}
#[doc = "DMAST (rw) register accessor: an alias for `Reg<DMAST_SPEC>`"]
pub type DMAST = crate::Reg<dmast::DMAST_SPEC>;
#[doc = "DMAC Module Activation Register"]
pub mod dmast;
