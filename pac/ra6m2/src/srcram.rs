#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x56c0 - Filter Coefficient Table \\[%s\\]"]
    pub srcfctr: [SRCFCTR; 5552],
}
#[doc = "SRCFCTR (rw) register accessor: an alias for `Reg<SRCFCTR_SPEC>`"]
pub type SRCFCTR = crate::Reg<srcfctr::SRCFCTR_SPEC>;
#[doc = "Filter Coefficient Table \\[%s\\]"]
pub mod srcfctr;
