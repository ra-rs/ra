#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Temperature Sensor Control Register"]
    pub tscr: TSCR,
}
#[doc = "TSCR (rw) register accessor: an alias for `Reg<TSCR_SPEC>`"]
pub type TSCR = crate::Reg<tscr::TSCR_SPEC>;
#[doc = "Temperature Sensor Control Register"]
pub mod tscr;
