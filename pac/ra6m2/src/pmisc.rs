#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Ethernet Control Register"]
    pub pfenet: PFENET,
    _reserved1: [u8; 0x02],
    #[doc = "0x03 - Write-Protect Register"]
    pub pwpr: PWPR,
}
#[doc = "PFENET (rw) register accessor: an alias for `Reg<PFENET_SPEC>`"]
pub type PFENET = crate::Reg<pfenet::PFENET_SPEC>;
#[doc = "Ethernet Control Register"]
pub mod pfenet;
#[doc = "PWPR (rw) register accessor: an alias for `Reg<PWPR_SPEC>`"]
pub type PWPR = crate::Reg<pwpr::PWPR_SPEC>;
#[doc = "Write-Protect Register"]
pub mod pwpr;
