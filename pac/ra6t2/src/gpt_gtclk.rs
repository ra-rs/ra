#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - General PWM Timer Clock Control Register"]
    pub gtclkcr: GTCLKCR,
}
#[doc = "GTCLKCR (rw) register accessor: an alias for `Reg<GTCLKCR_SPEC>`"]
pub type GTCLKCR = crate::Reg<gtclkcr::GTCLKCR_SPEC>;
#[doc = "General PWM Timer Clock Control Register"]
pub mod gtclkcr;
