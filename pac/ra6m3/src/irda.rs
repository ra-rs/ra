#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IrDA Control Register"]
    pub ircr: IRCR,
}
#[doc = "IRCR (rw) register accessor: an alias for `Reg<IRCR_SPEC>`"]
pub type IRCR = crate::Reg<ircr::IRCR_SPEC>;
#[doc = "IrDA Control Register"]
pub mod ircr;
