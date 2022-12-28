#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x03],
    #[doc = "0x03 - Write-Protect Register"]
    pub pwpr: PWPR,
}
#[doc = "PWPR (rw) register accessor: an alias for `Reg<PWPR_SPEC>`"]
pub type PWPR = crate::Reg<pwpr::PWPR_SPEC>;
#[doc = "Write-Protect Register"]
pub mod pwpr;
