#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Output Phase Switching Control Register"]
    pub opscr: OPSCR,
}
#[doc = "OPSCR (rw) register accessor: an alias for `Reg<OPSCR_SPEC>`"]
pub type OPSCR = crate::Reg<opscr::OPSCR_SPEC>;
#[doc = "Output Phase Switching Control Register"]
pub mod opscr;
